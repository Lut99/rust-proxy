//  MAIN.rs
//    by Lut99
//
//  Created:
//    25 Apr 2024, 21:57:37
//  Last edited:
//    05 May 2024, 09:28:02
//  Auto updated?
//    Yes
//
//  Description:
//!   Entrypoint to the `rust-proxy` binary.
//

use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use error_trace::{trace, ErrorTrace as _};
use humanlog::{DebugMode, HumanLogger};
use log::{debug, error, info};
use rust_proxy::config::Config;
use serializable::Serializable as _;
use tokio::net::TcpListener;
use tokio::runtime::{Builder, Runtime};
use tokio::signal::unix::{signal, Signal, SignalKind};
#[cfg(feature = "https")]
use tokio_rustls::TlsAcceptor;


/***** CONSTANTS *****/
/// The number of seconds that we wait for background tasks to exist.
const RUNTIME_EXIT_TIMEOUT_S: u64 = 10;





/***** ARGUMENTS *****/
/// Defines the arguments to the binary.
#[derive(Debug, Parser)]
struct Arguments {
    /// If given, enables full logging (i.e., TRACE-level debugging).
    #[clap(long, global = true)]
    trace: bool,

    /// The path where the configuration is loaded from.
    #[clap(short, long, default_value = "./proxy.yml")]
    config_path: PathBuf,
}





/***** LIBRARY *****/
fn main() {
    /* SETUP */
    // Parse arguments
    let args = Arguments::parse();

    // Setup the logger
    if let Err(err) = HumanLogger::terminal(if args.trace { DebugMode::Full } else { DebugMode::Debug }).init() {
        eprintln!("WARNING: Failed to setup logger: {err} (no logging for this session)");
    }
    info!("{} - v{}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));

    // Load the config
    debug!("Loading proxy config...");
    let config: Config = match Config::from_path(&args.config_path) {
        Ok(config) => config,
        Err(err) => {
            error!("{}", trace!(("Failed to load proxy config file '{}'", args.config_path.display()), err));
            std::process::exit(1);
        },
    };
    debug!("Loaded config with {} hostname map(s)", config.hostnames.len());
    // Hack: let's make it static, we won't ever load another one anyway.
    let config: &'static Config = Box::leak(Box::new(config));

    // Load the not-found file
    let not_found: &'static Vec<u8> = match config.load_not_found_file() {
        Ok(file) => file,
        Err(err) => {
            error!("{}", err.trace());
            std::process::exit(1);
        },
    };

    // Load certificates
    #[cfg(feature = "https")]
    let tls_config: &'static TlsAcceptor = match config.load_certstore() {
        Ok(config) => Box::leak(Box::new(TlsAcceptor::from(config))),
        Err(err) => {
            error!("{}", err.trace());
            std::process::exit(1);
        },
    };

    // Build the tokio runtime
    debug!("Building tokio runtime...");
    let rt: Runtime = match Builder::new_multi_thread().enable_io().build() {
        Ok(rt) => rt,
        Err(err) => {
            error!("{}", trace!(("Failed to build tokio runtime"), err));
            std::process::exit(1);
        },
    };

    // Build the HTTP listener
    let http_addr: SocketAddr = SocketAddr::new(config.address, config.http_port);
    debug!("Binding HTTP listener to '{http_addr}'...");
    let http_listener: TcpListener = match rt.block_on(TcpListener::bind(http_addr)) {
        Ok(listener) => listener,
        Err(err) => {
            error!("{}", trace!(("Failed to bind HTTP listener to '{http_addr}'"), err));
            std::process::exit(1);
        },
    };

    // Build the HTTPS listener
    #[cfg(feature = "https")]
    let https_addr: SocketAddr = SocketAddr::new(config.address, config.https_port);
    #[cfg(feature = "https")]
    let https_listener: TcpListener = {
        debug!("Binding HTTPS listener to '{https_addr}'...");
        match rt.block_on(TcpListener::bind(https_addr)) {
            Ok(listener) => listener,
            Err(err) => {
                error!("{}", trace!(("Failed to bind HTTPS listener to '{https_addr}'"), err));
                std::process::exit(1);
            },
        }
    };

    // Build a SIGINT handler
    debug!("Registering SIGINT handler...");
    let mut sigint_handler: Signal = match rt.block_on(async move { signal(SignalKind::interrupt()) }) {
        Ok(handler) => handler,
        Err(err) => {
            error!("{}", trace!(("Failed to register SIGINT handler"), err));
            std::process::exit(1);
        },
    };

    // Build a SIGTERM handler
    debug!("Registering SIGTERM handler...");
    let mut sigterm_handler: Signal = match rt.block_on(async move { signal(SignalKind::terminate()) }) {
        Ok(handler) => handler,
        Err(err) => {
            error!("{}", trace!(("Failed to register SIGTERM handler"), err));
            std::process::exit(1);
        },
    };

    /* GAME LOOP */
    if let Err(err) = rt.block_on(async move {
        // Log that we made it
        #[cfg(not(feature = "https"))]
        info!("Initialization complete; now serving at '{http_addr}' (HTTP)...");
        #[cfg(feature = "https")]
        info!("Initialization complete; now serving at '{http_addr}' (HTTP) and '{https_addr}' (HTTPS)...");

        // Run the game loop and switch on the specific handler
        loop {
            #[cfg(not(feature = "https"))]
            tokio::select! {
                // HTTP
                res = http_listener.accept() => match res {
                    Ok((conn, addr)) => {
                        debug!("Received incoming connection on HTTP listener from '{addr}'");
                        tokio::spawn(rust_proxy::handlers::handle_http(config, not_found, addr, conn));
                    },
                    Err(err) => error!("{}", trace!(("Failed to accept incoming HTTP connection on port {}", config.http_port), err)),
                },

                // Signal handlers
                _ = sigint_handler.recv() => {
                    info!("Received SIGINT, terminating...");
                    break;
                }
                _ = sigterm_handler.recv() => {
                    info!("Received SIGTERM, terminating...");
                    break;
                }
            }

            #[cfg(feature = "https")]
            tokio::select! {
                // HTTP
                res = http_listener.accept() => match res {
                    Ok((conn, addr)) => {
                        debug!("Received incoming connection on HTTP listener from '{addr}'");
                        tokio::spawn(rust_proxy::handlers::handle_http(config, not_found, addr, conn));
                    },
                    Err(err) => error!("{}", trace!(("Failed to accept incoming HTTP connection on port {}", config.http_port), err)),
                },

                // HTTPS
                res = https_listener.accept() => match res {
                    Ok((conn, addr)) => {
                        debug!("Received incoming connection on HTTPS listener from '{addr}'");
                        tokio::spawn(rust_proxy::handlers::handle_https(config, not_found, tls_config, addr, conn));
                    },
                    Err(err) => error!("{}", trace!(("Failed to accept incoming HTTPS connection on port {}", config.https_port), err)),
                },

                // Signal handlers
                _ = sigint_handler.recv() => {
                    info!("Received SIGINT, terminating...");
                    break;
                }
                _ = sigterm_handler.recv() => {
                    info!("Received SIGTERM, terminating...");
                    break;
                }
            }
        }

        // OK, we made it
        Ok::<_, std::convert::Infallible>(())
    }) {
        error!("{}", trace!(("A fatal error occurred"), err));
        std::process::exit(1);
    }

    /* CLEANUP */
    // Drop the runtime, to be sure nothing is using the config anymore
    debug!("Shutting down runtime ({RUNTIME_EXIT_TIMEOUT_S}s timeout)...");
    rt.shutdown_timeout(Duration::from_millis(RUNTIME_EXIT_TIMEOUT_S * 1000));

    // Free the borrowed values before exiting
    // SAFETY: Getting back ownership is OK, as the functions borrowing it are futures that are guaranteed to no longer exist due to the `shutdown_timeout()`-call.
    #[cfg(feature = "https")]
    drop(unsafe { Box::from_raw((tls_config as *const TlsAcceptor) as *mut TlsAcceptor) });
    // SAFETY: Getting back ownership is OK, as the functions borrowing it are futures that are guaranteed to no longer exist due to the `shutdown_timeout()`-call.
    drop(unsafe { Box::from_raw((not_found as *const Vec<u8>) as *mut Vec<u8>) });
    // SAFETY: Getting back ownership is OK, as the functions borrowing it are futures that are guaranteed to no longer exist due to the `shutdown_timeout()`-call.
    drop(unsafe { Box::from_raw((config as *const Config) as *mut Config) });

    // Done
    info!("Done.");
}
