use crate::Transport;
use async_trait::async_trait;
use axum::{Router, http::{HeaderValue, Method}};
use tower_http::cors::CorsLayer;
use ferrox_errors::AppError;

pub struct HttpTransport {
    pub router: Router,
    pub port: u16,
}

impl HttpTransport {
    pub fn new(router: Router, port: u16) -> Self {
        Self { router, port }
    }

    /// Enforces a strict Zero Trust CORS policy.
    /// Only allows the explicitly provided domains (e.g. "https://frontend.com").
    pub fn with_strict_cors(mut self, allowed_origins: Vec<&str>) -> Self {
        let origins = allowed_origins
            .into_iter()
            .map(|o| o.parse::<HeaderValue>().unwrap())
            .collect::<Vec<_>>();

        let cors = CorsLayer::new()
            .allow_origin(origins)
            .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
            .allow_headers(tower_http::cors::Any);
        
        self.router = self.router.layer(cors);
        self
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
