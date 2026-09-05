use ferrox_config::load_config;
use ferrox_logger::LoggerConfig;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub server_port: u16,
    pub jwt_secret: String,
    pub logger: LoggerConfig,
}

impl AppConfig {
    pub fn load() -> Self {
        // Automatically merges default.toml, environment toml, and env vars prefixed with FERROX_
        load_config("FERROX_").expect("Failed to load application configuration")
    }
}
