use axum::Router;
use std::net::SocketAddr;
use tokio::signal;
use tracing::{info, error};
use yalc_errors::AppError;

pub struct YalcApp {
    router: Router,
    port: u16,
}

impl YalcApp {
    pub fn new(router: Router) -> Self {
        Self {
            router,
            port: 3000,
        }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub async fn start(self) -> Result<(), AppError> {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        
        info!("Starting YalcApp on {}", addr);

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| AppError::InternalServerError(e.into()))?;

        axum::serve(listener, self.router.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .map_err(|e| AppError::InternalServerError(e.into()))?;

        info!("YalcApp stopped gracefully.");
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
    println!("yalc-app initialized: Provides YalcApp bootstrap server.");
}
