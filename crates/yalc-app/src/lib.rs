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

    pub async fn start(mut self) -> Result<(), AppError> {
        // Add security middlewares (Helmet equivalent)
        use tower_http::{
            catch_panic::CatchPanicLayer,
            cors::CorsLayer,
            set_header::SetResponseHeaderLayer,
            timeout::TimeoutLayer,
        };
        use axum::http::HeaderValue;
        use std::time::Duration;

        self.router = self.router
            .layer(sentry_tower::NewSentryLayer::new_from_top())
            .layer(sentry_tower::SentryHttpLayer::with_transaction())
            .layer(TimeoutLayer::new(Duration::from_secs(15)))
            .layer(CatchPanicLayer::new())
            .layer(CorsLayer::permissive()) // In a real app, configure this tightly
            .layer(SetResponseHeaderLayer::overriding(
                axum::http::header::STRICT_TRANSPORT_SECURITY,
                HeaderValue::from_static("max-age=31536000; includeSubDomains"),
            ))
            .layer(SetResponseHeaderLayer::overriding(
                axum::http::header::X_FRAME_OPTIONS,
                HeaderValue::from_static("DENY"),
            ))
            .layer(SetResponseHeaderLayer::overriding(
                axum::http::header::X_CONTENT_TYPE_OPTIONS,
                HeaderValue::from_static("nosniff"),
            ));

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
