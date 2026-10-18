use std::path::Path;
use std::io;
use super::write_file;

pub fn scaffold(name: &str, target_dir: &Path) -> io::Result<()> {
    // 1. Ferrox.toml
    let ferrox_toml = format!(r#"[package]
name = "{name}"
version = "0.1.0"
template = "clean-service"
runtime = "rust"
description = "Ferrox Hexagonal Domain-Driven Microservice"

[architecture]
pattern = "hexagonal-ddd"
pure_domain = true
enforce_purity = true

[security]
paseto_v4 = true
sentinel_entropy_threshold = 4.8

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
thiserror = "1.0"
uuid = {{ version = "1.4", features = ["v4", "serde"] }}
chrono = {{ version = "0.4", features = ["serde"] }}
tokio = {{ version = "1.0", features = ["full"] }}
axum = "0.7"

[dev-dependencies]
tokio-test = "0.4"
"#);
    write_file(&target_dir.join("Cargo.toml"), &cargo_toml)?;

    // 3. .coderabbit.yaml
    let coderabbit = r#"# CodeRabbit configuration for Ferrox Ecosystem
language: en-US
tone_instructions: "Rigorous senior staff architect review enforcing Ferrox DDD and zero debug debris."
reviews:
  high_level_summary: true
  poem: false
  review_status: true
  collapse_walkthrough: false
  auto_review:
    enabled: true
    drafts: false
chat:
  auto_reply: true
"#;
    write_file(&target_dir.join(".coderabbit.yaml"), coderabbit)?;

    // 4. .github/workflows/ci.yml
    let ci_yml = r#"name: Ferrox CI & Quality Gate

on:
  push:
    branches: [ master, main ]
  pull_request:
    branches: [ master, main ]

jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Cargo Format
        run: cargo fmt -- --check
      - name: Cargo Clippy
        run: cargo clippy -- -D warnings
      - name: Run Test Suite
        run: cargo test --all
"#;
    write_file(&target_dir.join(".github/workflows/ci.yml"), ci_yml)?;

    // 5. Dockerfile (Distroless <25MB)
    let dockerfile = format!(r#"# Multi-stage Distroless Container Build (<25MB)
FROM rust:1.80-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src/ ./src/
RUN cargo build --release

FROM gcr.io/distroless/static-debian12:nonroot
WORKDIR /app
COPY --from=builder /app/target/release/{name} /app/service
USER nonroot:nonroot
EXPOSE 8080
ENTRYPOINT ["/app/service"]
"#);
    write_file(&target_dir.join("Dockerfile"), &dockerfile)?;

    // 6. docs/ARCHITECTURE.md
    let docs = format!(r#"# {name} Architecture Specification (Hexagonal DDD)

## 1. Domain Boundary & Invariants
This service adheres to the pure Domain-Driven Design (DDD) model:
- **Pure Domain (`src/domain/`)**: Zero framework or database dependencies.
- **Application (`src/application/`)**: CQRS Commands and Queries.
- **Infrastructure (`src/infrastructure/`)**: Axum controllers, adapters, repositories.

## 2. Invariant Rules
1. Order quantities must be strictly positive ($> 0$).
2. Transitions are monotonic and validated against internal state enums.
3. All domain events are emitted asynchronously via CQRS ports.
"#);
    write_file(&target_dir.join("docs/ARCHITECTURE.md"), &docs)?;

    // 7. Source code: Pure Domain, Application, Infrastructure, Main
    let domain_model = r#"//! Pure Domain Aggregate Root - Zero ORM or HTTP Framework Dependencies

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("Quantity must be strictly positive")]
    InvalidQuantity,
    #[error("Cannot cancel an order that is already completed")]
    AlreadyCompleted,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Approved,
    Cancelled,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderAggregate {
    id: Uuid,
    customer_id: String,
    quantity: u32,
    status: OrderStatus,
}

impl OrderAggregate {
    pub fn new(customer_id: impl Into<String>, quantity: u32) -> Result<Self, DomainError> {
        if quantity == 0 {
            return Err(DomainError::InvalidQuantity);
        }
        Ok(Self {
            id: Uuid::new_v4(),
            customer_id: customer_id.into(),
            quantity,
            status: OrderStatus::Pending,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn status(&self) -> &OrderStatus {
        &self.status
    }

    pub fn approve(&mut self) -> Result<(), DomainError> {
        if self.status == OrderStatus::Cancelled {
            return Err(DomainError::AlreadyCompleted);
        }
        self.status = OrderStatus::Approved;
        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), DomainError> {
        if self.status == OrderStatus::Approved {
            return Err(DomainError::AlreadyCompleted);
        }
        self.status = OrderStatus::Cancelled;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_creation_invariant() {
        let order = OrderAggregate::new("cust_123", 5).unwrap();
        assert_eq!(*order.status(), OrderStatus::Pending);
        assert!(OrderAggregate::new("cust_123", 0).is_err());
    }

    #[test]
    fn test_order_state_transitions() {
        let mut order = OrderAggregate::new("cust_456", 2).unwrap();
        assert!(order.approve().is_ok());
        assert_eq!(*order.status(), OrderStatus::Approved);
        assert!(order.cancel().is_err());
    }
}
"#;
    write_file(&target_dir.join("src/domain/mod.rs"), domain_model)?;

    let app_handlers = r#"//! Application Layer - CQRS Command Handlers

use crate::domain::{DomainError, OrderAggregate};

pub struct CreateOrderCommand {
    pub customer_id: String,
    pub quantity: u32,
}

pub struct OrderApplicationService;

impl OrderApplicationService {
    pub fn handle_create(cmd: CreateOrderCommand) -> Result<OrderAggregate, DomainError> {
        OrderAggregate::new(cmd.customer_id, cmd.quantity)
    }
}
"#;
    write_file(&target_dir.join("src/application/mod.rs"), app_handlers)?;

    let lib_rs = r#"pub mod domain;
pub mod application;
"#;
    write_file(&target_dir.join("src/lib.rs"), lib_rs)?;

    let main_rs = format!(r#"//! {name} Entry Point
use {name}::application::{{CreateOrderCommand, OrderApplicationService}};

#[tokio::main]
async fn main() {{
    println!("🚀 Launching Ferrox Clean Service: {name}");
    let order = OrderApplicationService::handle_create(CreateOrderCommand {{
        customer_id: "demo_customer".to_string(),
        quantity: 10,
    }}).expect("Order creation failed");
    println!("✅ Order created with ID: {{}}", order.id());
}}
"#);
    write_file(&target_dir.join("src/main.rs"), &main_rs)?;

    Ok(())
}
