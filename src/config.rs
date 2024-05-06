//  CONFIG.rs
//    by Lut99
//
//  Created:
//    25 Apr 2024, 22:25:21
//  Last edited:
//    06 May 2024, 18:57:18
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the config layout (and functions to load it) for the proxy
//!   service.
//

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FResult};
use std::io::BufReader;
use std::net::IpAddr;
use std::path::PathBuf;
#[cfg(feature = "https")]
use std::sync::Arc;
use std::{error, fs};

use serde::{Deserialize, Serialize};
pub use serializable::yaml::Error as YamlError;
use serializable::yaml::Serializer as YamlSerializer;
use serializable::Serializable;
#[cfg(feature = "https")]
use tokio_rustls::rustls::server::{ClientHello, ResolvesServerCert};
#[cfg(feature = "https")]
use tokio_rustls::rustls::sign::CertifiedKey;
#[cfg(feature = "https")]
use tokio_rustls::rustls::ServerConfig;


/***** ERRORS *****/
/// Defines the error returned by [`Config`]'s non-[`serializable`] functions.
#[derive(Debug)]
pub enum Error {
    /// Empty certificate file given.
    CertificateEmpty { hostname: String, path: PathBuf },
    /// Failed to open the given certificate file.
    CertificateOpen { hostname: String, path: PathBuf, err: std::io::Error },
    /// Failed to read & parse the given certificate file.
    CertificateParse { hostname: String, path: PathBuf, err: std::io::Error },
    /// A given private key was unsupported.
    PrivateKeyDecode { hostname: String, path: PathBuf, err: tokio_rustls::rustls::Error },
    /// Empty private key file given.
    PrivateKeyEmpty { hostname: String, path: PathBuf },
    /// Failed to open the given private key file.
    PrivateKeyOpen { hostname: String, path: PathBuf, err: std::io::Error },
    /// Failed to read & parse the given private key file.
    PrivateKeyParse { hostname: String, path: PathBuf, err: std::io::Error },
    /// Failed to read the not found file at the given path.
    NotFoundRead { path: PathBuf, err: std::io::Error },
}
impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Error::*;
        match self {
            CertificateEmpty { hostname, path } => write!(f, "No certificates in certificate file '{}' for hostname '{}'", path.display(), hostname),
            CertificateOpen { hostname, path, .. } => write!(f, "Failed to load certificate file '{}' for hostname '{}'", path.display(), hostname),
            CertificateParse { hostname, path, .. } => write!(f, "Failed to read certificate file '{}' for hostname '{}'", path.display(), hostname),
            PrivateKeyDecode { hostname, path, .. } => {
                write!(f, "Failed to decode private key in private key file '{}' for hostname '{}'", path.display(), hostname)
            },
            PrivateKeyEmpty { hostname, path } => write!(f, "No private keys in private key file '{}' for hostname '{}'", path.display(), hostname),
            PrivateKeyOpen { hostname, path, .. } => write!(f, "Failed to load private key file '{}' for hostname '{}'", path.display(), hostname),
            PrivateKeyParse { hostname, path, .. } => write!(f, "Failed to read private key file '{}' for hostname '{}'", path.display(), hostname),
            NotFoundRead { path, .. } => write!(f, "Failed to load not found file at '{}'", path.display()),
        }
    }
}
impl error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn 'static + error::Error)> {
        use Error::*;
        match self {
            CertificateEmpty { .. } => None,
            CertificateOpen { err, .. } => Some(err),
            CertificateParse { err, .. } => Some(err),
            PrivateKeyDecode { err, .. } => Some(err),
            PrivateKeyEmpty { .. } => None,
            PrivateKeyOpen { err, .. } => Some(err),
            PrivateKeyParse { err, .. } => Some(err),
            NotFoundRead { err, .. } => Some(err),
        }
    }
}





/***** HELPERS *****/
/// Defines a custom certificate resolver based on loaded config files.
#[cfg(feature = "https")]
#[derive(Debug)]
struct CertificateResolver {
    /// The store of certificates we loaded.
    certstore: HashMap<String, Arc<CertifiedKey>>,
}
#[cfg(feature = "https")]
impl ResolvesServerCert for CertificateResolver {
    fn resolve(&self, client_hello: ClientHello) -> Option<Arc<CertifiedKey>> {
        // Attempt to get a certificate
        let name: &str = client_hello.server_name()?;
        Some(self.certstore.get(name)?.clone())
    }
}





/***** LIBRARY *****/
/// Defines the config layout for the proxy service.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// The address on which to bind the server.
    pub address: IpAddr,
    /// The ports on which we create openings.
    pub ports:   Vec<u16>,

    /// Stores hostname -> other hostname maps.
    pub hostnames: HashMap<String, String>,
    /// Stores hostname -> certificate to use.
    #[cfg(feature = "https")]
    pub certs: HashMap<String, CertPath>,
    /// The hostname on which any certbot server might live.
    #[cfg(feature = "certbot")]
    pub certbot_hostname: String,

    /// Stores the path to a file to send back if no mapping is found.
    pub not_found_file: PathBuf,
}
impl Config {
    /// Loads the not found file from the path defined in the config.
    ///
    /// Do _not_ call this function multiple times willy-nilly, as the resulting reference is not dropped automatically and may result in memory being leaked and growing full.
    ///
    /// # Returns
    /// A loaded, and "made static" (don't ask me how please) array of bytes encoding the file to send back when a hostname is not found.
    ///
    /// # Errors
    /// This function fails if we failed to load the file from the `not_found_file` path in the config.
    pub fn load_not_found_file(&self) -> Result<&'static Vec<u8>, Error> {
        match fs::read(&self.not_found_file) {
            // Leak the file as a static box before returning, in order to make life with multi-threaded tokio much easer.
            // NOTE: Don't forget to re-capture and drop before exiting!
            Ok(file) => Ok(Box::leak(Box::new(file))),
            Err(err) => Err(Error::NotFoundRead { path: self.not_found_file.clone(), err }),
        }
    }

    /// Loads a rustls [`ServerConfig`] from the internally specified certificate- and private key paths.
    ///
    /// # Returns
    /// A loaded [`ServerConfig`] wrapped in an [`Arc`].
    ///
    /// # Errors
    /// This function fails if we failed to load the file from the `not_found_file` path in the config.
    #[cfg(feature = "https")]
    pub fn load_certstore(&self) -> Result<Arc<ServerConfig>, Error> {
        use std::fs::File;

        use log::debug;
        use rustls_pki_types::{CertificateDer, PrivateKeyDer};
        use tokio_rustls::rustls::crypto;
        use tokio_rustls::rustls::sign::SigningKey;

        let mut store: HashMap<String, Arc<CertifiedKey>> = HashMap::with_capacity(self.certs.len());
        for (hostname, path) in self.certs.iter() {
            // Attempt to read the certificates
            let certs: Vec<CertificateDer> = {
                // Open the file
                let mut handle: BufReader<File> = match File::open(&path.certificate) {
                    Ok(handle) => BufReader::new(handle),
                    Err(err) => return Err(Error::CertificateOpen { hostname: hostname.clone(), path: path.certificate.clone(), err }),
                };

                // Use the crate to read the certificates
                let certs: Vec<CertificateDer> = match rustls_pemfile::certs(&mut handle).collect::<Result<Vec<CertificateDer>, std::io::Error>>() {
                    Ok(certs) => certs,
                    Err(err) => return Err(Error::CertificateParse { hostname: hostname.clone(), path: path.certificate.clone(), err }),
                };
                if !certs.is_empty() {
                    certs
                } else {
                    return Err(Error::CertificateEmpty { hostname: hostname.clone(), path: path.certificate.clone() });
                }
            };

            // Attempt to read the private keys
            let key: PrivateKeyDer = {
                // Open the file
                let mut handle: BufReader<File> = match File::open(&path.key) {
                    Ok(handle) => BufReader::new(handle),
                    Err(err) => return Err(Error::PrivateKeyOpen { hostname: hostname.clone(), path: path.key.clone(), err }),
                };

                // Use the crate to read the certificates
                match rustls_pemfile::private_key(&mut handle) {
                    Ok(Some(key)) => key,
                    Ok(None) => return Err(Error::PrivateKeyEmpty { hostname: hostname.clone(), path: path.key.clone() }),
                    Err(err) => return Err(Error::PrivateKeyParse { hostname: hostname.clone(), path: path.key.clone(), err }),
                }
            };

            // Convert it to an appropriate key
            let key: Arc<dyn SigningKey> = match crypto::aws_lc_rs::sign::any_supported_type(&key) {
                Ok(key) => key,
                Err(err) => return Err(Error::PrivateKeyDecode { hostname: hostname.clone(), path: path.key.clone(), err }),
            };

            // OK, add them
            debug!("Loaded {} certificate(s), 1 key(s) for '{}'", certs.len(), hostname);
            store.insert(hostname.clone(), Arc::new(CertifiedKey { cert: certs, key, ocsp: None }));
        }

        // Build a server config
        let tls_config: Arc<ServerConfig> =
            Arc::new(ServerConfig::builder().with_no_client_auth().with_cert_resolver(Arc::new(CertificateResolver { certstore: store })));

        // Done, leak the pointer
        Ok(tls_config)
    }
}
impl Serializable<YamlSerializer<Config>> for Config {}



/// Defines how to define a certificate/keypair.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CertPath {
    /// The path of the certificate file
    #[serde(alias = "crt", alias = "cert")]
    certificate: PathBuf,
    /// The path fo the key file.
    key: PathBuf,
}
