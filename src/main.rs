//  MAIN.rs
//    by Lut99
//
//  Created:
//    25 Apr 2024, 21:57:37
//  Last edited:
//    06 May 2024, 19:26:46
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
use tokio::task::JoinSet;
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
    let acceptor: &'static TlsAcceptor = match config.load_certstore() {
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

    // Build the HTTP listeners
    let code: i32 = rt.block_on(async move {
        let mut listeners: JoinSet<Result<(), i32>> = JoinSet::new();
        for port in &config.ports {
            listeners.spawn(Box::pin(async move {
                let addr: SocketAddr = SocketAddr::new(config.address, *port);
                debug!("Binding listener to '{addr}'...");
                let listener: TcpListener = match TcpListener::bind(addr).await {
                    Ok(listener) => listener,
                    Err(err) => {
                        error!("{}", trace!(("Failed to bind listener to '{addr}'"), err));
                        return Err(1i32);
                    },
                };

                info!("Now serving at '{addr}'");
                loop {
                    match listener.accept().await {
                        Ok((conn, addr)) => {
                            debug!("Received incoming connection on HTTP listener from '{addr}'");
                            #[cfg(not(feature = "https"))]
                            tokio::spawn(rust_proxy::handlers::handle_http(config, not_found, addr, conn));
                            #[cfg(feature = "https")]
                            tokio::spawn(rust_proxy::handlers::handle_https(config, not_found, acceptor, addr, conn));
                        },
                        Err(err) => error!("{}", trace!(("Failed to accept incoming HTTP connection on port {}", addr.port()), err)),
                    }
                }
            }));
        }

        // Build a SIGINT handler
        listeners.spawn(Box::pin(async move {
            // Create the signal handler
            debug!("Registering SIGINT handler...");
            let mut sigint_handler: Signal = match signal(SignalKind::interrupt()) {
                Ok(handler) => handler,
                Err(err) => {
                    error!("{}", trace!(("Failed to register SIGINT handler"), err));
                    return Err(1i32);
                },
            };

            // Block on it
            sigint_handler.recv().await;
            Ok(())
        }));

        // Build a SIGTERM handler
        listeners.spawn(Box::pin(async move {
            // Create the signal handler
            debug!("Registering SIGTERM handler...");
            let mut sigint_handler: Signal = match signal(SignalKind::terminate()) {
                Ok(handler) => handler,
                Err(err) => {
                    error!("{}", trace!(("Failed to register SIGTERM handler"), err));
                    return Err(1i32);
                },
            };

            // Block on it
            sigint_handler.recv().await;
            Ok(())
        }));



        /* GAME LOOP */
        // Simply await the first to return
        match listeners.join_next().await.transpose() {
            Ok(Some(Ok(_))) => 0i32,
            Ok(Some(Err(code))) => code,
            Ok(None) => unreachable!(),
            Err(err) => {
                error!("{}", trace!(("Failed to join handles"), err));
                1
            },
        }
    });



    /* CLEANUP */
    // Drop the runtime, to be sure nothing is using the config anymore
    debug!("Shutting down runtime ({RUNTIME_EXIT_TIMEOUT_S}s timeout)...");
    rt.shutdown_timeout(Duration::from_millis(RUNTIME_EXIT_TIMEOUT_S * 1000));

    // Free the borrowed values before exiting
    // SAFETY: Getting back ownership is OK, as the functions borrowing it are futures that are guaranteed to no longer exist due to the `shutdown_timeout()`-call.
    #[cfg(feature = "https")]
    drop(unsafe { Box::from_raw((acceptor as *const TlsAcceptor) as *mut TlsAcceptor) });
    // SAFETY: Getting back ownership is OK, as the functions borrowing it are futures that are guaranteed to no longer exist due to the `shutdown_timeout()`-call.
    drop(unsafe { Box::from_raw((not_found as *const Vec<u8>) as *mut Vec<u8>) });
    // SAFETY: Getting back ownership is OK, as the functions borrowing it are futures that are guaranteed to no longer exist due to the `shutdown_timeout()`-call.
    drop(unsafe { Box::from_raw((config as *const Config) as *mut Config) });

    // Done
    info!("Done.");
    std::process::exit(code);
}
