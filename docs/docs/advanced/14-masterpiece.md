---
sidebar_position: 14
---

# The Enterprise Masterpiece

In Phase 22, `Ferrox` officially graduated from a framework into an **Enterprise Ecosystem**. 

We implemented 8 advanced modules that are strictly required for Cloud-Native, High-Availability, and microservices architectures.

## 1. Cron Jobs (`ferrox-schedule`)
Easily schedule background tasks using cron syntax.
```rust
use ferrox_schedule::Scheduler;

let mut scheduler = Scheduler::new().await.unwrap();
scheduler.add_job("0 0 12 * * * *", || {
    println!("Running daily billing batch...");
}).await.unwrap();
scheduler.start().await.unwrap();
```

## 2. Server-Sent Events (`ferrox-sse`)
Stream unidirectional data to clients (like ChatGPT does) natively without the overhead of WebSockets. 

## 3. Kubernetes Probes (`ferrox-health`)
Deploying to Kubernetes? `Ferrox` automatically exposes `/healthz` (Liveness) and `/readyz` (Readiness) endpoints. K8s will automatically restart your pod if the Database or Redis disconnects.

## 4. OpenTelemetry Tracing (`ferrox-tracing`)
Trace requests across microservices. Every request gets an OTLP trace ID, which can be exported to Datadog or Jaeger, allowing you to visualize exactly where latency is occurring in your distributed system.

## 5. Circuit Breaker (`ferrox-circuit-breaker`)
When calling external APIs (e.g., Stripe or Google), if they go down, your app will not crash from cascading timeouts. The `Circuit Breaker` temporarily blocks requests ("Opens the circuit") and tests recovery ("Half-Open") before allowing traffic back.

## 6. Hierarchical Configs (`ferrox-config`)
Stop relying on loose `.env` files. `Ferrox` merges `default.toml`, environment-specific `[env].toml`, and environment variables, strictly validating them against your Rust structs. If a config is missing, the server crashes immediately (Fail-Fast).

## 7. i18n Localization (`ferrox-i18n`)
Built-in support for intercepting `Accept-Language` headers and translating API responses dynamically.

## 8. Migrations Engine (`ferrox-migrations`)
Integrated `sea-orm-migration` runner to apply versioned database schemas automatically on boot.
