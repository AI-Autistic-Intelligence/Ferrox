use std::sync::Arc;
use tokio::signal;
use tokio::task::JoinHandle;
use tracing::{info, error};
use yalc_errors::AppError;
use yalc_transports::Transport;

pub struct YalcApp {
    transports: Vec<Arc<dyn Transport>>,
}

impl Default for YalcApp {
    fn default() -> Self {
        Self::new()
    }
}

impl YalcApp {
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
        info!("Starting YalcApp multi-transport system...");

        if self.transports.is_empty() {
            return Err(AppError::InternalServerError(
                "Cannot start YalcApp: no transports configured!".into(),
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
    println!("yalc-app initialized: Multi-Transport YalcApp bootstrap ready.");
}
