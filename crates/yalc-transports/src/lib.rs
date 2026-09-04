use async_trait::async_trait;
use yalc_errors::AppError;

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

#[cfg(feature = "grpc")]
pub mod grpc;

#[cfg(feature = "ftp")]
pub mod ftp;

pub fn setup() {
    println!("yalc-transports initialized: Multi-Transport system ready.");
}
