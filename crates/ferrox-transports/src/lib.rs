//! # Ferrox Transports (`ferrox-transports`)
//!
//! `ferrox-transports` defines the core `Transport` trait and protocol server builders (HTTP Axum, gRPC Tonic, WebSockets)
//! managed by `FerroxApp`.
//!
//! ## Key Features
//! - 🌐 **`Transport` Trait**: Common async interface (`start()`, `name()`) for all network protocol listeners.
//! - 🚀 **`HttpTransport` Builder**: Axum HTTP server wrapper with CORS, timeouts, and fallback routing.

use async_trait::async_trait;
use ferrox_errors::AppError;

#[async_trait]
pub trait Transport: Send + Sync {
    /// Starts the transport. This is usually a blocking async operation
    /// (e.g. running an axum server or a tonic grpc server).
    async fn start(&self) -> Result<(), AppError>;
    
    /// Returns the name of the transport for logging purposes.
    fn name(&self) -> &'static str;
}

#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "http")]
pub use http::HttpTransport;

#[cfg(feature = "http")]
pub mod ws;
#[cfg(feature = "http")]
pub use ws::WsTransport;

#[cfg(feature = "grpc")]
pub mod grpc;

#[cfg(feature = "ftp")]
pub mod ftp;

pub fn setup() {
    println!("ferrox-transports initialized: Multi-Transport system ready.");
}