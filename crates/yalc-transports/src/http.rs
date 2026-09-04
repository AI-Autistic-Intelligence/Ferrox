use crate::Transport;
use async_trait::async_trait;
use axum::Router;
use yalc_errors::AppError;

pub struct HttpTransport {
    pub router: Router,
    pub port: u16,
}

impl HttpTransport {
    pub fn new(router: Router, port: u16) -> Self {
        Self { router, port }
    }
}

#[async_trait]
impl Transport for HttpTransport {
    async fn start(&self) -> Result<(), AppError> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = tokio::net::TcpListener::bind(&addr).await.map_err(|e| {
            AppError::InternalError(format!("Failed to bind HTTP port {}: {}", self.port, e))
        })?;
        
        println!("🚀 Starting HTTP Transport on {}", addr);
        axum::serve(listener, self.router.clone()).await.map_err(|e| {
            AppError::InternalError(format!("HTTP Server Error: {}", e))
        })?;
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "HTTP/REST"
    }
}
