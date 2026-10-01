use crate::Transport;
use async_trait::async_trait;
use ferrox_errors::AppError;

/// Tonic gRPC server transport
pub struct GrpcTransport {
    pub port: u16,
}

impl GrpcTransport {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}

#[async_trait]
impl Transport for GrpcTransport {
    async fn start(&self) -> Result<(), AppError> {
        let addr = format!("0.0.0.0:{}", self.port).parse().unwrap();
        println!("🚀 Starting gRPC Transport on {}", addr);
        
        let (_, rx) = tokio::sync::oneshot::channel::<()>();
        
        // This is a minimal Tonic server setup using a healthcheck router to prevent immediate termination.
        // In a real application, the concrete gRPC service handlers would be added here via `add_service`.
        let router = tonic::transport::Server::builder()
            .add_service(tonic_health::server::health_reporter().1);
            
        router.serve_with_shutdown(addr, async {
            rx.await.ok();
        }).await.map_err(|e| AppError::InternalError(e.to_string()))?;
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "gRPC (Tonic)"
    }
}
