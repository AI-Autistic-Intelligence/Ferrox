---
sidebar_position: 6
---

# ⚙️ Typed Configuration Management

Applications need environment-specific configurations (port numbers, database URLs, JWT secret keys, log levels) that change between Development, Staging, and Production environments.

Ferrox provides typed, safe configuration loading via `ferrox-config`, supporting `.env` files, TOML files, environment variable overrides, and secret masking via `secrecy::Secret`.

---

## 1. Defining Application Configuration

Create a strongly typed configuration struct using `serde::Deserialize`:

```rust
use serde::Deserialize;
use secrecy::Secret;
use ferrox_logger::LoggerConfig;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server_port: u16,
    pub jwt_secret: String,
    pub database_url: String,
    pub logger: LoggerConfig,
}

impl AppConfig {
    pub fn load() -> Self {
        // Loads from .env or default fallback values
        let port = std::env::var("FERROX_SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("Invalid FERROX_SERVER_PORT");

        let jwt_secret = std::env::var("FERROX_JWT_SECRET")
            .unwrap_or_else(|_| "super_secret_default_key_32bytes!!".to_string());

        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/ferrox_dev".to_string());

        Self {
            server_port: port,
            jwt_secret,
            database_url,
            logger: LoggerConfig::default(),
        }
    }
}
```

---

## 2. Protecting Sensitive Secrets with `secrecy::Secret`

Raw string secrets stored in memory can accidentally leak via debug logs or crash dumps. Ferrox recommends wrapping sensitive tokens in `secrecy::Secret`:

```rust
use secrecy::{Secret, ExposeSecret};

pub struct SecurityService {
    jwt_secret: Secret<String>,
}

impl SecurityService {
    pub fn new(raw_secret: String) -> Self {
        Self {
            jwt_secret: Secret::new(raw_secret),
        }
    }

    pub fn get_key(&self) -> &str {
        self.jwt_secret.expose_secret()
    }
}
```

---

## 3. Injecting Configuration into State

Pass the loaded `AppConfig` into your Axum router state:

```rust
use axum::{extract::State, routing::get, Router};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
}

async fn get_config_info(State(state): State<AppState>) -> String {
    format!("Server running on port: {}", state.config.server_port)
}
```

---

## 4. ✅ Best Practices

- **Never hardcode secrets in code repositories**: Store secrets in environment variables or cloud key vaults (AWS Secrets Manager / HashiCorp Vault).
- **Use `secrecy` wrappers**: Prevent sensitive credentials from leaking into stdout logs.
