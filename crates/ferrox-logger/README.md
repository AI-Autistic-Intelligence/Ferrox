# Ferrox Logger (`ferrox-logger`)

[![Crates.io](https://img.shields.io/crates/v/ferrox-logger.svg)](https://crates.io/crates/ferrox-logger)
[![Documentation](https://docs.rs/ferrox-logger/badge.svg)](https://docs.rs/ferrox-logger)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

`ferrox-logger` sets up production-ready, structured JSON logging, Sentry telemetry, and cryptographic SHA-256 Merkle root log chain auditing for Ferrox and Rust applications.

---

## 🔑 Key Features

- 📝 **Structured JSON Output**: Standardized log format with timestamps, severity levels, module targets, and OpenTelemetry trace IDs.
- 🔗 **`MerkleLogLedger`**: Cryptographic SHA-256 Merkle root computation over structured log blocks for tamper-evident auditing and regulatory compliance.
- 🚨 **Sentry Integration**: Automatic panic and critical error event reporting to Sentry APM.
- 🎛️ **Environmental Filtering**: Configurable log level thresholds via `RUST_LOG` environment variables.

---

## 🚀 Quickstart Usage

Add `ferrox-logger` to your `Cargo.toml`:

```toml
[dependencies]
ferrox-logger = "0.1.2"
tracing = "0.1"
```

### 1. Setup Structured JSON Logging & Sentry

```rust,no_run
use ferrox_logger::{setup_logger, LoggerConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _sentry_guard = setup_logger(LoggerConfig {
        service_name: "my-service".to_string(),
        environment: "production".to_string(),
        otlp_endpoint: None,
        sentry_dsn: None,
        log_level: "info".to_string(),
    })?;

    tracing::info!("Application booted successfully");
    Ok(())
}
```

### 2. Cryptographic Merkle Log Audit Chain (`MerkleLogLedger`)

```rust
use ferrox_logger::merkle::MerkleLogLedger;

fn main() {
    let mut ledger = MerkleLogLedger::new();
    ledger.record_log("sys_audit", "User auth success: usr_102");
    ledger.record_log("sys_audit", "Role updated: usr_102 -> admin");

    let root_hash = ledger.compute_merkle_root();
    println!("Tamper-evident Merkle Root: {}", root_hash);
}
```

---

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
