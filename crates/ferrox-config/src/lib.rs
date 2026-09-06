//! # Ferrox Config (`ferrox-config`)
//!
//! `ferrox-config` handles strongly-typed application configuration loading from environment variables (`.env`) and TOML files,
//! integrating `secrecy` to prevent accidental logging of passwords and API keys.
//!
//! ## Key Features
//! - 🔒 **`SecretString` Integration**: Protect sensitive database URIs and API keys from accidental printing in logs.
//! - 📄 **Multi-Source Merging**: Read from `.env`, environment variables, and `config/default.toml` hierarchically.

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
        .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

    info!("Loaded strongly-typed configuration for environment: {}", environment);

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestConfig {
        host: String,
        port: u16,
    }

    #[test]
    fn test_load_config_env_override() {
        // Set an env var that should be picked up by Figment
        std::env::set_var("APP_HOST", "127.0.0.1");
        std::env::set_var("APP_PORT", "8080");

        let config: Result<TestConfig, _> = load_config("APP_");
        assert!(config.is_ok());
        
        let config = config.unwrap();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
    }
}