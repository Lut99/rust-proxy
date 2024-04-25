//  HANDLERS.rs
//    by Lut99
//
//  Created:
//    25 Apr 2024, 22:31:03
//  Last edited:
//    26 Apr 2024, 00:00:58
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

use crate::config::Config;


/***** CONSTANTS *****/
/// The length of the HTTP line buffer.
const BUFFER_LEN: usize = 16384;





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
pub async fn write_not_found_file(client: SocketAddr, not_found_html: &'static [u8], mut socket: impl AsyncWrite + Unpin) {
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





/***** LIBRARY *****/
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
        } else if carriage_return && *b == b'\n' || i == buf_len - 1 {
            // It's a full line, parse as the host header
            let line: &[u8] = if i < buf_len - 1 { &buf[ptr..i - 1] } else { &buf[ptr..buf_len] };
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
    let mut target: Option<SocketAddr> = None;
    'resume: {
        if let Some(host) = config.hostnames.get(host) {
            // Attempt to resolve the hostname to some address
            let addrs = match host.to_socket_addrs() {
                Ok(addrs) => addrs,
                Err(err) => {
                    error!(
                        "{}",
                        trace!(("Failed to resolve '{host}' to a hostname or hostname and port; pretending we don't know the hostname"), err)
                    );
                    break 'resume;
                },
            };

            // Find the target
            for addr in addrs {
                if target.is_none() {
                    target = Some(addr);
                } else {
                    error!("Host '{host}' resolves to multiple hostnames; pretending we don't know the hostname");
                    break 'resume;
                }
            }
        }
    }
    let target: SocketAddr = match target {
        Some(target) => target,
        None => {
            debug!("[{client}] Client provided unknown hostname '{host}'");

            // Send the file back
            write_not_found_file(client, not_found_html, &mut socket).await;
            drop(socket);
            return;
        },
    };

    // Open a new connection
    debug!("[{client} -> {target}] Opening connection...");
    let mut target_sock: TcpStream = match TcpStream::connect(target).await {
        Ok(socket) => socket,
        Err(err) => {
            error!("[{client}] {}", trace!(("Failed to connect to resolved host '{target}'"), err));
            return;
        },
    };

    // First, stream the original request
    debug!("[{client} -> {target}] Flushing buffered request content...");
    if let Err(err) = target_sock.write_all(&buf[..buf_len]).await {
        error!("[{client}] {}", trace!(("Failed to forward buffered request of {buf_len} bytes to resolved host '{target}'"), err));
        return;
    }

    // Alright, now just copy everything
    debug!("[{client} -> {target}] Setting up bidirectional copy...");
    if let Err(err) = tokio::io::copy_bidirectional(&mut socket, &mut target_sock).await {
        error!("[{client} -> {target}] {}", trace!(("Failed to copy bidirectional"), err));
        return;
    }

    // Done
    debug!("[{client} -> {target}] Connection completed.");
}



/// Handles incoming HTTPS connections.
///
/// # Arguments
/// - `config`: The [`Config`] that provides us with proxy mappings and certificate paths and the like.
/// - `not_found_html`: Some bytes to send back if nothing was found.
/// - `client`: The address of the newly connected client.
/// - `socket`: The accepted client socket we're connecting over. Note that it's implemented in the abstract to be compatible with processing HTTPS connections.
///
/// # Errors
/// This function may error if something went wrong.
#[cfg(feature = "https")]
pub async fn handle_https(config: &'static Config, not_found_html: &'static [u8], client: SocketAddr, socket: impl AsyncRead + AsyncWrite) {}
