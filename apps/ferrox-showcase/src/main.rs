mod config;
mod modules;

use axum::Router;
use std::sync::Arc;
use secrecy::Secret;
use tracing::info;

use ferrox_app::FerroxApp;
use ferrox_transports::http::HttpTransport;
use ferrox_logger::setup_logger;
use ferrox_security::PasetoAuth;

use crate::config::AppConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load Configuration
    std::env::set_var("FERROX_SERVER_PORT", "3000");
    std::env::set_var("FERROX_JWT_SECRET", "super_secret_key_that_is_32_bytes_long_12345678901234567890");
    std::env::set_var("FERROX_LOGGER.SERVICE_NAME", "ferrox-showcase");
    
    let config = AppConfig::load();

    // 2. Setup Structured Logger & Tracing
    let _sentry_guard = setup_logger(config.logger.clone())
        .expect("Failed to initialize logger");

    info!("🚀 Booting Ferrox Showcase App...");

    // 3. Initialize Security (PASETO)
    let secret = Secret::new(config.jwt_secret);
    let paseto_auth = Arc::new(PasetoAuth::new(secret).unwrap());

    // 4. Assemble Router with Health Probes & OpenAPI Swagger Docs
    let app_router = Router::new()
        .merge(modules::health::router())
        .merge(modules::docs::router())
        .nest("/api/v1/auth", modules::auth::router(paseto_auth));

    // 5. Setup HTTP Transport with Zero Trust CORS
    let http_transport = HttpTransport::new(app_router, config.server_port)
        .with_strict_cors(vec!["http://localhost:8080", "https://myfrontend.com"]);

    info!("⚡ Ferrox HTTP Transport listening on port {}", config.server_port);
    info!("📖 OpenAPI Swagger UI available at http://localhost:{}/docs", config.server_port);
    info!("🩺 Health probes available at /health/live and /health/ready");

    // 6. Bootstrap Ferrox App Lifecycle Manager (handles SIGINT/SIGTERM gracefully)
    info!("Starting Ferrox lifecycle manager...");
    let app = FerroxApp::new()
        .add_transport(http_transport);

    app.start().await?;

    Ok(())
}
