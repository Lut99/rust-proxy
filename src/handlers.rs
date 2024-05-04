//  HANDLERS.rs
//    by Lut99
//
//  Created:
//    25 Apr 2024, 22:31:03
//  Last edited:
//    04 May 2024, 09:12:33
//  Auto updated?
//    Yes
//
//  Description:
//!   Implements the handler(s) for new connections.
//

use std::net::{SocketAddr, ToSocketAddrs as _};

use chrono::Utc;
use error_trace::trace;
use log::{debug, error};
use tokio::io::{AsyncRead, AsyncReadExt as _, AsyncWrite, AsyncWriteExt as _};
use tokio::net::TcpStream;
#[cfg(feature = "https")]
use tokio_rustls::rustls::ServerConfig;

use crate::config::Config;


/***** CONSTANTS *****/
/// The length of the HTTP line buffer.
const BUFFER_LEN: usize = 8192;

/// Defines the certbot file prefix.
// SAFETY: Msut be valid UTF-8
#[cfg(feature = "certbot")]
const CERTBOT_PATH_PREFIX: &[u8] = b"/.well-known/acme-challenge/";





/***** HELPER FUNCTIONS *****/
/// Sends back the not-found file to the given socket, with appropriate headers.
///
/// # Arguments
/// - `client`: The address of the client, for debugging.
/// - `not_found_html`: The bytes of the file to send back.
/// - `socket`: The client's socket to write back on.
///
/// # Errors
/// This function can fail if we failed to write to the given `socket`.
///
/// Note that only warnings are emitted, in that case.
async fn write_not_found_file(client: SocketAddr, not_found_html: &'static [u8], mut socket: impl AsyncWrite + Unpin) {
    debug!("[{client}] Sending back not-found file of {} bytes", not_found_html.len());

    // Send back the status code
    if let Err(err) = socket.write_all(b"HTTP/1.1 200 OK\r\n").await {
        error!("[{client}] ABORT: {}", trace!(("Failed to send back HTTP header"), err));
        return;
    }

    // Send back the date
    if let Err(err) = socket.write_all(Utc::now().format("Date: %a, %d %b %Y %H:%M:%S GMT\r\n").to_string().as_bytes()).await {
        error!("[{client}] ABORT: {}", trace!(("Failed to send back HTTP header"), err));
        return;
    }

    // Send back the server string
    if let Err(err) = socket.write_all(concat!("Server: ", env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"), "\r\n").as_bytes()).await {
        error!("[{client}] ABORT: {}", trace!(("Failed to send back HTTP header"), err));
        return;
    }

    // Send back the content length
    if let Err(err) = socket.write_all(b"Content-Length: ").await {
        error!("[{client}] ABORT: {}", trace!(("Failed to send back HTTP header"), err));
        return;
    }
    if let Err(err) = socket.write_all(not_found_html.len().to_string().as_bytes()).await {
        error!("[{client}] ABORT: {}", trace!(("Failed to send back HTTP header"), err));
        return;
    }
    if let Err(err) = socket.write_all(b"\r\n").await {
        error!("[{client}] ABORT: {}", trace!(("Failed to send back HTTP header"), err));
        return;
    }

    // Send back the content type
    if let Err(err) = socket.write_all(b"Content-Type: text/html\r\n").await {
        error!("[{client}] ABORT: {}", trace!(("Failed to send back HTTP header"), err));
        return;
    }

    // Finally, send back that they shouldn't talk to us
    if let Err(err) = socket.write_all(b"Connection: Closed\r\n\r\n").await {
        error!("[{client}] ABORT: {}", trace!(("Failed to send back HTTP header"), err));
        return;
    }

    // Then write the body
    if let Err(err) = socket.write_all(not_found_html).await {
        error!("[{client}] ABORT: {}", trace!(("Failed to send back not-found file of {} bytes", not_found_html.len()), err));
        return;
    }
    if let Err(err) = socket.flush().await {
        error!("[{client}] {}", trace!(("Failed to flush client connection"), err));
        return;
    }
}

/// Given a connection, proxies it to the given host.
///
/// This function blocks until either side closes the connection.
///
/// # Arguments
/// - `client`: The address of the newly connected client.
/// - `socket`: The socket-like to redirect.
/// - `target`: Some string hostname(:port) pair to resolve to a target address.
/// - `buf`: Any buffered content to flush to the remote host first.
///
/// # Errors
/// This function fails if the redirect fails at any point.\
async fn redirect(client: SocketAddr, mut socket: impl AsyncRead + AsyncWrite + Unpin, target: &str, buf: &[u8]) {
    // Attempt to resolve the hostname to some address
    debug!("[{client}] Resolving target address '{target}'...");
    let addrs = match target.to_socket_addrs() {
        Ok(addrs) => addrs,
        Err(err) => {
            error!("[{client}] ABORT: {}", trace!(("Failed to resolve '{target}' to a hostname or hostname and port; aborting redirect"), err));
            drop(socket);
            return;
        },
    };

    // Find the target
    let mut target_ip: Option<SocketAddr> = None;
    for addr in addrs {
        debug!("[{client}] Possible resolution of '{target}': '{addr}'");
        if let Some(prev) = &mut target_ip {
            // Prefer IPv6 over IPv4
            if prev.is_ipv6() && addr.is_ipv4() {
                *prev = addr;
            }
        } else {
            // Just set it
            target_ip = Some(addr);
        }
    }
    let target: SocketAddr = match target_ip {
        Some(target) => target,
        None => {
            error!("[{client}] ABORT: Failed to resolve '{target}' to a hostname or hostname and port; aborting redirect");
            drop(socket);
            return;
        },
    };

    // Open a new connection
    debug!("[{client} -> {target}] Opening connection...");
    let mut target_sock: TcpStream = match TcpStream::connect(target).await {
        Ok(socket) => socket,
        Err(err) => {
            error!("[{client}] ABORT: {}", trace!(("Failed to connect to resolved host '{target}'"), err));
            return;
        },
    };

    // First, stream the original request
    debug!("[{client} -> {target}] Flushing buffered request content...");
    if let Err(err) = target_sock.write_all(buf).await {
        error!("[{client}] ABORT: {}", trace!(("Failed to forward buffered request of {} bytes to resolved host '{}'", buf.len(), target), err));
        return;
    }

    // Alright, now just copy everything
    debug!("[{client} -> {target}] Setting up bidirectional copy...");
    if let Err(err) = tokio::io::copy_bidirectional(&mut socket, &mut target_sock).await {
        error!("[{client} -> {target}] ABORT: {}", trace!(("Failed to copy bidirectional"), err));
        return;
    }

    // Done
    debug!("[{client} -> {target}] Connection completed.");
    drop(target_sock);
    drop(socket);
}





/***** LIBRARY *****/
/// Handles re

/// Handles incoming HTTP connections.
///
/// # Arguments
/// - `config`: The [`Config`] that provides us with proxy mappings.
/// - `not_found_html`: Some bytes to send back if nothing was found.
/// - `client`: The address of the newly connected client.
/// - `socket`: The accepted client socket we're connecting over. Note that it's implemented in the abstract to be compatible with processing HTTPS connections.
///
/// # Errors
/// This function may error if something went wrong.
pub async fn handle_http(
    config: &'static Config,
    not_found_html: &'static [u8],
    client: SocketAddr,
    mut socket: impl AsyncRead + AsyncWrite + Unpin,
) {
    // First, read only one chunk and try to find the HTTP buffer in there
    let mut buf: [u8; BUFFER_LEN] = [0; BUFFER_LEN];
    let buf_len: usize = match socket.read(&mut buf).await {
        Ok(len) => len,
        Err(err) => {
            debug!("[{client}] ABORT: {}", trace!(("Failed to read stream"), err));
            return;
        },
    };
    if buf_len == 0 {
        debug!("[{client}] ABORT: Client closed connection before anything is sent.");
        return;
    }

    // Read the bytes line-by-line
    let mut ptr: usize = 0;
    let mut carriage_return: bool = false;
    let mut host: Option<&[u8]> = None;
    for (i, b) in buf[..buf_len].iter().enumerate() {
        // Handle the CRLF scanning
        if !carriage_return && *b == b'\r' {
            carriage_return = true;
            continue;
        } else if carriage_return && *b == b'\n' {
            let line: &[u8] = &buf[ptr..i - 1];

            // It's a full line, parse as the host header _or_ the GET thing
            if &line[..6] == b"Host: " {
                let mut hostname: &[u8] = &line[6..];

                // Optionally discard the port
                if let Some(pos) = hostname.iter().enumerate().find_map(|(i, b)| if *b == b':' { Some(i) } else { None }) {
                    hostname = &hostname[..pos];
                }

                // Set the hostname
                host = Some(hostname);
                break;
            }
            #[cfg(feature = "certbot")]
            if &line[..4] == b"GET " {
                let mut path: &[u8] = &line[4..];

                // Strip any ending ' HTTP/X'
                // NOTE: The needle is reversed because we're searching back-to-front
                let needle: &[u8] = b"/PTTH ";
                let mut needle_i: usize = 0;
                for (i, b) in path.iter().enumerate().rev() {
                    if *b == needle[needle_i] {
                        needle_i += 1;
                        if needle_i == needle.len() {
                            // Found it and strip it
                            path = &path[..i];
                            break;
                        }
                    } else {
                        needle_i = 0;
                    }
                }

                // The rest is the path; check if it's the certbot path
                if path.len() >= CERTBOT_PATH_PREFIX.len() && &path[..CERTBOT_PATH_PREFIX.len()] == CERTBOT_PATH_PREFIX {
                    // SAFETY: Works as long as 'CERTBOT_PATH_PREFIX' is valid UTF-8
                    debug!(
                        "[{client}] Found GET-request to '{}', redirecting to certbot at '{}'",
                        unsafe { std::str::from_utf8_unchecked(path) },
                        config.certbot_hostname
                    );

                    // Catch the request, and sent it to the certbot client instead
                    redirect(client, &mut socket, &config.certbot_hostname, &buf[..buf_len]).await;
                    return;
                }
            }

            // Update the ptr
            ptr = i + 1;
        }
        carriage_return = false;
    }
    let host: &[u8] = match host {
        Some(host) => host,
        None => {
            debug!("[{client}] ABORT: Client did not provide hostname within the first {BUFFER_LEN} bytes.");
            return;
        },
    };
    let host: &str = match std::str::from_utf8(host) {
        Ok(host) => host,
        Err(err) => {
            debug!("[{client}] ABORT: {}", trace!(("Client provided non-UTF-8 hostname '{:#X?}'", host), err));
            return;
        },
    };
    debug!("[{client}] Client provided hostname '{host}'");

    // Attempt to find the hostname in the map
    let target: &str = match config.hostnames.get(host) {
        Some(target) => target.as_str(),
        None => {
            debug!("[{client}] Unknown client-provided hostname '{host}'");

            // Send the file back
            write_not_found_file(client, not_found_html, &mut socket).await;
            drop(socket);
            return;
        },
    };

    // The rest is left as a redirect
    redirect(client, socket, target, &buf[..buf_len]).await;
}



/// Handles incoming HTTPS connections.
///
/// # Arguments
/// - `config`: The [`Config`] that provides us with proxy mappings and certificate paths and the like.
/// - `not_found_html`: Some bytes to send back if nothing was found.
/// - `tls_config`: Some [`ServerConfig`] that configures how we accept TLS requests.
/// - `client`: The address of the newly connected client.
/// - `socket`: The accepted client socket we're connecting over. Note that it's implemented in the abstract to be compatible with processing HTTPS connections.
///
/// # Errors
/// This function may error if something went wrong.
#[cfg(feature = "https")]
pub async fn handle_https(
    config: &'static Config,
    not_found_html: &'static [u8],
    tls_config: &'static ServerConfig,
    client: SocketAddr,
    socket: impl AsyncRead + AsyncWrite,
) {
}
