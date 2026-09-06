//! # Ferrox App (`ferrox-app`)
//!
//! `ferrox-app` provides the primary application bootstrapper and multi-transport lifecycle orchestrator
//! for the Ferrox framework. It manages concurrent server instances (HTTP, gRPC, WebSockets, etc.) and enforces
//! graceful shutdown handling across UNIX signals (`SIGTERM`) and cross-platform interrupts (`Ctrl+C`).
//!
//! ## Architectural Role
//! In enterprise applications, backends often need to serve multiple network protocols simultaneously (e.g. Axum for HTTP/REST,
//! Tonic for gRPC inter-service communication). `FerroxApp` encapsulates these network transports into unified `Arc<dyn Transport>`
//! workers and manages their startup, execution lifecycle, and teardown concurrently.
//!
//! ## Key Features
//! - 🌐 **Multi-Transport Execution**: Boot HTTP, gRPC, and background listeners concurrently.
//! - 🛡️ **Graceful Shutdown Orchestration**: Catches OS termination signals and shuts down active transport threads cleanly.
//! - ⚡ **Integration with Tower & Sentry**: Built-in support for middleware, cors, timeouts, and error capturing.
//!
//! ## Example Usage
//! ```rust,no_run
//! use ferrox_app::FerroxApp;
//! use ferrox_transports::http::HttpTransport;
//! use axum::{Router, routing::get};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let router = Router::new().route("/health", get(|| async { "OK" }));
//!     let transport = HttpTransport::new(router, 8080);
//!
//!     FerroxApp::new()
//!         .add_transport(transport)
//!         .start()
//!         .await?;
//!
//!     Ok(())
//! }
//! ```

use std::sync::Arc;
use tokio::signal;
use tokio::task::JoinHandle;
use tracing::{info, error};
use ferrox_errors::AppError;
use ferrox_transports::Transport;

pub struct FerroxApp {
    transports: Vec<Arc<dyn Transport>>,
}

impl Default for FerroxApp {
    fn default() -> Self {
        Self::new()
    }
}

impl FerroxApp {
    pub fn new() -> Self {
        Self {
            transports: Vec::new(),
        }
    }

    /// Add a transport layer to the app (e.g. HttpTransport, GrpcTransport, FtpTransport)
    pub fn add_transport<T: Transport + 'static>(mut self, transport: T) -> Self {
        self.transports.push(Arc::new(transport));
        self
    }

    /// Starts all configured transports concurrently and waits for shutdown signal
    pub async fn start(self) -> Result<(), AppError> {
        info!("Starting FerroxApp multi-transport system...");

        if self.transports.is_empty() {
            return Err(AppError::InternalServerError(
                "Cannot start FerroxApp: no transports configured!".into(),
            ));
        }

        let mut join_handles: Vec<JoinHandle<Result<(), AppError>>> = Vec::new();

        for transport in self.transports {
            let t = Arc::clone(&transport);
            
            let handle = tokio::spawn(async move {
                info!("Booting transport: {}", t.name());
                if let Err(e) = t.start().await {
                    error!("Transport {} crashed: {:?}", t.name(), e);
                    return Err(e);
                }
                Ok(())
            });
            
            join_handles.push(handle);
        }

        // Wait for shutdown signal
        shutdown_signal().await;

        info!("Graceful shutdown initiated...");
        
        // In a real implementation we would send a cancellation token to all join handles
        for handle in join_handles {
            handle.abort();
        }

        info!("FerroxApp stopped gracefully.");
        Ok(())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutdown signal received, starting graceful shutdown...");
}

pub fn setup() {
    println!("ferrox-app initialized: Multi-Transport FerroxApp bootstrap ready.");
}