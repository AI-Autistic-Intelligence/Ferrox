//! # Ferrox Logger (`ferrox-logger`)
//!
//! `ferrox-logger` sets up production-ready, structured JSON logging and Sentry telemetry for Ferrox applications.
//! Built on top of `tracing-subscriber` and `sentry`, it provides unified log formatting, environmental level filtering,
//! and automated crash reports.
//!
//! ## Architectural Context
//! Containerized environments (Kubernetes, AWS ECS, Docker) require logs to be emitted in structured JSON format via stdout
//! for centralized aggregation (Elasticsearch, Datadog, Loki). `ferrox-logger` ensures all log events retain trace IDs,
//! timestamps, and module metadata.
//!
//! ## Key Features
//! - 📝 **Structured JSON Output**: Standardized log format with timestamp, severity level, target module, and dynamic fields.
//! - 🚨 **Sentry Integration**: Automatic reporting of critical error events to Sentry APM.
//! - 🎛️ **Environment-Driven Filtering**: Configurable log level thresholds via `RUST_LOG` environment variables.
//!
//! ## Example Usage
//! ```rust,no_run
//! use ferrox_logger::{setup_logger, LoggerConfig};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let _sentry_guard = setup_logger(LoggerConfig::default())?;
//!     tracing::info!("Application booted successfully");
//!     Ok(())
//! }
//! ```

use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::{self, Sampler};
use opentelemetry_sdk::Resource;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use ferrox_errors::AppError;

pub mod sanitizer;
pub mod dp;
pub mod weekly_report;
pub mod merkle;

pub struct LoggerConfig {
    pub service_name: String,
    pub environment: String,
    pub otlp_endpoint: Option<String>,
    pub sentry_dsn: Option<String>,
    pub log_level: String,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            service_name: "ferrox-app".to_string(),
            environment: "development".to_string(),
            otlp_endpoint: None,
            sentry_dsn: None,
            log_level: "info".to_string(),
        }
    }
}

pub fn setup_logger(config: LoggerConfig) -> Result<Option<sentry::ClientInitGuard>, AppError> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.log_level));
    
    // Default JSON formatting for standard output
    let formatting_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    let subscriber = Registry::default().with(env_filter).with(formatting_layer);

    // If Sentry DSN is provided, setup Sentry
    let mut sentry_guard = None;
    if let Some(dsn) = config.sentry_dsn {
        sentry_guard = Some(sentry::init((
            dsn,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                traces_sample_rate: 1.0,
                environment: Some(config.environment.into()),
                ..Default::default()
            },
        )));
    }

    let _ = subscriber.try_init();

    tracing::info!("ferrox-logger initialized: Structured JSON logging enabled for service '{}'.", config.service_name);
    Ok(sentry_guard)
}