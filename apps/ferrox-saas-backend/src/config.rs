use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseEngine {
    Sql,
    NoSql,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server_port: u16,
    pub environment: String,
    pub jwt_secret: String,
    pub database_engine: DatabaseEngine,
    pub database_url: String,
    pub redis_url: String,
    pub sentry_dsn: Option<String>,
    pub cors_origins: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_port: 8080,
            environment: "development".to_string(),
            jwt_secret: "super_secret_ferrox_paseto_key_32_bytes!!".to_string(),
            database_engine: DatabaseEngine::Sql,
            database_url: "postgres://postgres:postgres@localhost:5432/ferrox_saas".to_string(),
            redis_url: "redis://127.0.0.1:6379".to_string(),
            sentry_dsn: None,
            cors_origins: vec!["http://localhost:3000".to_string(), "http://localhost:8080".to_string()],
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        let port = std::env::var("FERROX_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);

        let jwt_secret = std::env::var("FERROX_JWT_SECRET")
            .unwrap_or_else(|_| "super_secret_ferrox_paseto_key_32_bytes!!".to_string());

        let engine = match std::env::var("FERROX_DB_ENGINE").as_deref() {
            Ok("nosql") | Ok("mongo") => DatabaseEngine::NoSql,
            _ => DatabaseEngine::Sql,
        };

        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ferrox_saas".to_string());

        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

        Self {
            server_port: port,
            environment: std::env::var("FERROX_ENV").unwrap_or_else(|_| "development".to_string()),
            jwt_secret,
            database_engine: engine,
            database_url,
            redis_url,
            sentry_dsn: std::env::var("SENTRY_DSN").ok(),
            cors_origins: vec!["http://localhost:3000".to_string(), "http://localhost:8080".to_string()],
        }
    }
}
