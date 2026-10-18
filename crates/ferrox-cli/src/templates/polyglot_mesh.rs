use std::path::Path;
use std::io;
use super::write_file;

pub fn scaffold(name: &str, target_dir: &Path) -> io::Result<()> {
    // 1. Ferrox.toml
    let ferrox_toml = format!(r#"[package]
name = "{name}"
version = "0.1.0"
template = "polyglot-mesh"
runtime = "rust"
description = "Ferrox Distributed Polyglot Event Mesh Node"

[mesh]
redpanda_brokers = ["localhost:19092"]
inbound_topic = "market.ticks.v1"
outbound_topic = "orders.matched.v1"
redis_url = "redis://localhost:6379"

[quality]
min_coverage = 100
forbid_debug = true
"#);
    write_file(&target_dir.join("Ferrox.toml"), &ferrox_toml)?;

    // 2. Cargo.toml
    let cargo_toml = format!(r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
tokio = {{ version = "1.0", features = ["full"] }}
uuid = {{ version = "1.4", features = ["v4", "serde"] }}
chrono = {{ version = "0.4", features = ["serde"] }}
"#);
    write_file(&target_dir.join("Cargo.toml"), &cargo_toml)?;

    // 3. docker-compose.yml
    let docker_compose = r#"version: '3.8'
services:
  redpanda:
    image: docker.redpanda.com/redpandadata/redpanda:v24.1.2
    container_name: local-mesh-redpanda
    command:
      - redpanda start
      - --kafka-addr internal://0.0.0.0:9092,external://0.0.0.0:19092
      - --advertise-kafka-addr internal://redpanda:9092,external://localhost:19092
      - --mode dev-container
      - --smp 1
      - --memory 1G
    ports:
      - "19092:19092"

  redis:
    image: redis:7-alpine
    container_name: local-mesh-redis
    ports:
      - "6379:6379"
"#;
    write_file(&target_dir.join("docker-compose.yml"), docker_compose)?;

    // 4. .coderabbit.yaml
    let coderabbit = r#"language: en-US
tone_instructions: "Enforce cloud-native event mesh standards: idempotency, CloudEvents v1.0, and distributed saga compensations."
"#;
    write_file(&target_dir.join(".coderabbit.yaml"), coderabbit)?;

    // 5. Dockerfile
    let dockerfile = format!(r#"FROM rust:1.80-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src/ ./src/
RUN cargo build --release

FROM gcr.io/distroless/static-debian12:nonroot
WORKDIR /app
COPY --from=builder /app/target/release/{name} /app/mesh-node
USER nonroot:nonroot
ENTRYPOINT ["/app/mesh-node"]
"#);
    write_file(&target_dir.join("Dockerfile"), &dockerfile)?;

    // 6. docs/ARCHITECTURE.md
    let docs = format!(r#"# {name} - Polyglot Event Mesh Architecture

## CloudEvents Specification
All inter-service messaging complies with CloudEvents 1.0 JSON format:
- Inbound: `market.ticks.v1`
- Outbound: `orders.matched.v1`
- Distributed Locking: Redis Redlock algorithm.
"#);
    write_file(&target_dir.join("docs/ARCHITECTURE.md"), &docs)?;

    // 7. Source code
    let cloudevent_rs = r#"//! CloudEvents 1.0 Specification Implementation

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CloudEvent<T> {
    pub specversion: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub source: String,
    pub id: String,
    pub time: String,
    pub datacontenttype: String,
    pub data: T,
}

impl<T> CloudEvent<T> {
    pub fn new(event_type: impl Into<String>, source: impl Into<String>, data: T) -> Self {
        Self {
            specversion: "1.0".to_string(),
            event_type: event_type.into(),
            source: source.into(),
            id: Uuid::new_v4().to_string(),
            time: Utc::now().to_rfc3339(),
            datacontenttype: "application/json".to_string(),
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct SampleData {
        symbol: String,
        price: f64,
    }

    #[test]
    fn test_cloudevent_serialization() {
        let event = CloudEvent::new(
            "market.tick.v1",
            "ferrox/mesh-node",
            SampleData { symbol: "BTC/USD".into(), price: 67400.0 },
        );
        let serialized = serde_json::to_string(&event).unwrap();
        assert!(serialized.contains("market.tick.v1"));
        assert!(serialized.contains("67400"));
    }
}
"#;
    write_file(&target_dir.join("src/cloudevent.rs"), cloudevent_rs)?;

    let lib_rs = r#"pub mod cloudevent;
"#;
    write_file(&target_dir.join("src/lib.rs"), lib_rs)?;

    let main_rs = format!(r#"//! {name} Polyglot Mesh Node
use {name}::cloudevent::CloudEvent;
use serde_json::json;

#[tokio::main]
async fn main() {{
    println!("🚀 Launching Ferrox Polyglot Mesh Node: {name}");
    let event = CloudEvent::new(
        "ferrox.mesh.node.ready",
        "{name}",
        json!({{"status": "ONLINE", "runtime": "rust"}}),
    );
    println!("📡 Broadcast event: {{}}", serde_json::to_string_pretty(&event).unwrap());
}}
"#);
    write_file(&target_dir.join("src/main.rs"), &main_rs)?;

    Ok(())
}
