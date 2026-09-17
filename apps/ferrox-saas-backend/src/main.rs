use ferrox_saas_backend::{config, modules, persistence};

use axum::{middleware, routing::get, Json, Router};
use ferrox_app::FerroxApp;
use ferrox_logger::{setup_logger, LoggerConfig};
use ferrox_transports::http::HttpTransport;
use serde_json::json;
use std::sync::Arc;
use tracing::info;

use crate::config::AppConfig;
use crate::modules::auth::PasetoAuth;
use crate::persistence::InMemoryUserRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::load();

    // Setup Ferrox Logger & Sentry Guard
    let _sentry_guard = setup_logger(LoggerConfig {
        service_name: "ferrox-saas-backend".to_string(),
        environment: config.environment.clone(),
        otlp_endpoint: None,
        sentry_dsn: config.sentry_dsn.clone(),
        log_level: "info".to_string(),
    }).expect("Failed to initialize Ferrox logger");

    info!("🚀 Booting Ferrox Enterprise SaaS Framework ({:?})...", config.database_engine);

    // Initialize Security (PASETO) & Persistence Repository
    let paseto_auth = Arc::new(PasetoAuth::new(config.jwt_secret.clone()));
    let user_repo = Arc::new(InMemoryUserRepository::new());

    // Assemble API Routes
    #[allow(unused_mut)]
    let mut api_routes = Router::new()
        .route("/health", get(|| async { Json(json!({ "status": "UP", "framework": "Ferrox Framework v0.1.2" })) }))
        .nest("/api/v1/auth", modules::auth::router(paseto_auth, user_repo.clone()))
        .nest("/api/v1/users", modules::users::router(user_repo.clone()))
        .nest("/api/v1/admin", modules::admin::router(user_repo.clone()))
        .nest("/api/v1/mailer", modules::mailer::router())
        .nest("/api/v1/notifications", modules::notifications::router());

    #[cfg(feature = "founder-suite")]
    {
        api_routes = api_routes.nest("/api/v1/founder", modules::founder::router());
    }

    let api_routes = api_routes
        .layer(middleware::from_fn(modules::observability::trace_request_lifecycle_middleware))
        .layer(middleware::from_fn(ferrox_guards::mandatory_compliance_middleware));

    // Configure Ferrox Multi-Transport HTTP Server
    let http_transport = HttpTransport::new(api_routes, config.server_port)
        .with_strict_cors(config.cors_origins.iter().map(|s| s.as_str()).collect());

    info!("⚡ Ferrox HTTP Server listening on port {}", config.server_port);

    // Start Ferrox Application Lifecycle Orchestrator
    FerroxApp::new()
        .add_transport(http_transport)
        .start()
        .await?;

    Ok(())
}
