# Ferrox Logger (`ferrox-logger`)

`ferrox-logger` sets up production-ready, structured JSON logging and Sentry telemetry for Ferrox applications.
Built on top of `tracing-subscriber` and `sentry`, it provides unified log formatting, environmental level filtering,
and automated crash reports.

## Architectural Context
Containerized environments (Kubernetes, AWS ECS, Docker) require logs to be emitted in structured JSON format via stdout
for centralized aggregation (Elasticsearch, Datadog, Loki). `ferrox-logger` ensures all log events retain trace IDs,
timestamps, and module metadata.

## Key Features
- 📝 **Structured JSON Output**: Standardized log format with timestamp, severity level, target module, and dynamic fields.
- 🚨 **Sentry Integration**: Automatic reporting of critical error events to Sentry APM.
- 🎛️ **Environment-Driven Filtering**: Configurable log level thresholds via `RUST_LOG` environment variables.

## Example Usage
```rust,no_run
use ferrox_logger::{setup_logger, LoggerConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _sentry_guard = setup_logger(LoggerConfig::default())?;
    tracing::info!("Application booted successfully");
    Ok(())
}
```
