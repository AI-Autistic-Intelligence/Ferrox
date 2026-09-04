use ferrox_errors::AppError;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;
use tracing::info;

/// Loads and validates configuration from `default.toml`, environment-specific `[env].toml`, 
/// and environment variables. Uses strongly typed deserialization to Fail-Fast on missing keys.
pub fn load_config<'a, T: Deserialize<'a>>(env_prefix: &str) -> Result<T, AppError> {
    let environment = std::env::var("FERROX_ENV").unwrap_or_else(|_| "development".into());

    let config: T = Figment::new()
        .merge(Toml::file("config/default.toml"))
        .merge(Toml::file(format!("config/{}.toml", environment)))
        .merge(Env::prefixed(env_prefix))
        .extract()
        .map_err(|e| AppError::InternalError(format!("Configuration Validation Failed: {}", e)))?;

    info!("Loaded strongly-typed configuration for environment: {}", environment);

    Ok(config)
}
