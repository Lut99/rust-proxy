//  CONFIG.rs
//    by Lut99
//
//  Created:
//    25 Apr 2024, 22:25:21
//  Last edited:
//    27 Apr 2024, 12:23:19
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the config layout (and functions to load it) for the proxy
//!   service.
//

use std::collections::HashMap;
use std::net::IpAddr;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
pub use serializable::yaml::Error;
use serializable::yaml::Serializer as YamlSerializer;
use serializable::Serializable;


/***** LIBRARY *****/
/// Defines the config layout for the proxy service.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// The address on which to bind the server.
    pub address:    IpAddr,
    /// The port where the HTTP server is hosted.
    pub http_port:  u16,
    /// The port where the HTTPS server is hosted.
    #[cfg(feature = "https")]
    pub https_port: u16,

    /// Stores hostname -> other hostname maps.
    pub hostnames: HashMap<String, String>,
    /// Stores hostname -> certificate to use.
    pub certs:     HashMap<String, CertPath>,

    /// Stores the path to a file to send back if no mapping is found.
    pub not_found_file: PathBuf,
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
