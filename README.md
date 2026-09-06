# ⚡ Ferrox Framework

<p align="center">
  <img src="docs/static/img/logo.jpg" alt="Ferrox Logo" width="200" />
</p>

<p align="center">
  <b>A Progressive, Enterprise-Grade Server-Side Framework for Rust</b><br/>
  <i>Bringing the Developer Experience of NestJS & Angular to the unmatched performance of Tokio & Axum.</i>
</p>

<p align="center">
  <a href="#-philosophy--architectural-rationale">Philosophy</a> •
  <a href="#-the-onion-request-pipeline">Onion Architecture</a> •
  <a href="#-crate-workspace-inventory">Crate Inventory</a> •
  <a href="#-end-to-end-quick-start">Quick Start</a> •
  <a href="#-documentation">Documentation</a> •
  <a href="https://discord.gg/Bx3CzGec7d">Discord</a> •
  <a href="https://www.reddit.com/r/Ferrox/">Reddit</a>
</p>

---

## 🎯 Philosophy & Architectural Rationale

In the modern server-side development landscape, frameworks like **NestJS** (Node.js) and **Spring Boot** (Java) popularized structured, maintainable, and modular backend architecture based on **Inversion of Control (IoC)**, **Dependency Injection**, and explicit architectural boundaries. However, as web systems scale to tens of millions of concurrent requests, single-threaded runtimes face CPU exhaustion while heavy garbage-collected virtual machines suffer from unpredictable latencies and massive memory footprints.

Conversely, while **Rust** provides unmatched memory safety, zero-cost abstractions, and extreme asynchronous throughput via **Tokio** and **Axum**, developers building production services often struggle with:
1. **Lack of Framework Structure**: Micro-frameworks leave architectural decisions (error handling, auth context, validation pipelines) entirely to the developer, leading to fragmented, non-standardized codebases across teams.
2. **Repetitive Boilerplate**: Hand-crafting HTTP extractors, validation checks, database mapping, and error conversions leads to developer fatigue.
3. **Resilience Gaps**: Implementing enterprise patterns like **Singleflight** (cache stampede prevention), **Circuit Breakers**, **Saga Orchestrators**, and **PASETO Token Engines** requires integrating dozens of disparate third-party crates.

### **Ferrox bridges the gap between rapid Developer Experience (DX) and extreme Rust performance.**

Built natively on top of [Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/), Ferrox provides an out-of-the-box, opinionated enterprise architecture for building production-ready, highly testable, zero-trust microservices and monolithic web applications.

### 🔑 Key Enterprise Highlights
- 🛡️ **Zero-Trust Security by Default**: Built-in PASETO v4 token translation engine, declarative `RequireRole` guards, and HMAC-SHA256 webhook signature verification.
- ⚡ **Cache Stampede Protection**: Integrated `ferrox-singleflight` pattern powered by Tokio broadcast channels to eliminate database dogpiling under heavy traffic spikes.
- 🔄 **Resilience & Fault Tolerance**: Redis-backed rate limiters, tri-state Circuit Breakers (`Closed`, `Open`, `HalfOpen`), and distributed locks (`Redlock`).
- 🧠 **Enterprise Architectures**: Decoupled `CommandBus` / `QueryBus` (CQRS), Saga distributed transaction orchestrators with rollback compensation, and async Domain Event buses.
- 🛠️ **Code Factory & AutoZod**: Automated generic REST CRUD router generation (`crud_router!`) and strongly-typed payload validation (`ValidatedJson<T>`).
- 📊 **Unified Observability**: Kubernetes liveness (`/healthz`) and readiness (`/readyz`) probes, Prometheus metrics exporter, and OpenTelemetry OTLP distributed tracing.

---

## 🧅 The Onion Request Pipeline

Ferrox enforces a strictly ordered **Onion Request Pipeline**. Requests pass through non-blocking, early-failing security, rate-limiting, and validation layers before ever touching business domain logic or database connection pools.

```
       +---------------------------------------------------------+
       |                   Incoming HTTP Stream                  |
       +---------------------------------------------------------+
                                    |
                                    v
       +---------------------------------------------------------+
       |   1. Global Middleware (Logging, Tracing, CORS)         |
       |      - Generates W3C Correlation IDs & Tracing Spans    |
       +---------------------------------------------------------+
                                    |
                                    v
       +---------------------------------------------------------+
       |   2. Rate Limiter & Circuit Breaker (Redis / In-Mem)     |
       |      - Rejects abusive traffic (HTTP 429 Too Many Req)  |
       |      - Prevents cascading downstream service failures   |
       +---------------------------------------------------------+
                                    |
                                    v
       +---------------------------------------------------------+
       |   3. Auth Guards (PASETO / JWT Claims Extraction)         |
       |      - Decrypts PASETO v4 local/public tokens           |
       |      - Enforces declarative RequireRole / Permissions   |
       +---------------------------------------------------------+
                                    |
                                    v
       +---------------------------------------------------------+
       |   4. Validation Pipe (AutoZod / ValidatedJson<T>)        |
       |      - Validates payload field constraints via validator |
       |      - Rejects invalid DTOs with HTTP 400 Bad Request   |
       +---------------------------------------------------------+
                                    |
                                    v
       +---------------------------------------------------------+
       |   5. Controller Route Handler                            |
       |      - Extracts validated DTOs, path params, & state    |
       +---------------------------------------------------------+
                                    |
                                    v
       +---------------------------------------------------------+
       |   6. Business Provider / Service Layer                  |
       |      - Singleflight cache stampede suppression          |
       |      - CQRS CommandBus / QueryBus execution             |
       |      - Saga Distributed Transaction Orchestration       |
       +---------------------------------------------------------+
                                    |
                                    v
       +---------------------------------------------------------+
       |   7. Data Persistence Layer                             |
       |      - Repository<Entity, Id> abstraction               |
       |      - SeaORM (SQL) / Mongo (BSON) / Redis (KV Cache)   |
       +---------------------------------------------------------+
```

---

## 📦 Crate Workspace Inventory

Ferrox is structured as a modular workspace containing 45 specialized crates categorized by domain:

| Category | Crate Name | Description | README |
|---|---|---|---|
| **Core Bootstrapping** | `ferrox-app` | Multi-transport server bootstrapper and graceful shutdown orchestrator | [Read README](crates/ferrox-app/README.md) |
| | `ferrox-errors` | Centralized `AppError`, `ErrorResponse`, and Axum `IntoResponse` | [Read README](crates/ferrox-errors/README.md) |
| | `ferrox-config` | Strongly-typed environment & TOML loader with `secrecy` protection | [Read README](crates/ferrox-config/README.md) |
| | `ferrox-types` | Standard domain types, type-safe `PublicId`, and `Pagination` helpers | [Read README](crates/ferrox-types/README.md) |
| | `ferrox-utils` | Shared utility functions, UTC date formatters, and string casing helpers | [Read README](crates/ferrox-utils/README.md) |
| **Abstractions & DX** | `ferrox-validation` | `ValidatedJson<T>` extractor powered by `validator` constraint checks | [Read README](crates/ferrox-validation/README.md) |
| | `ferrox-guards` | Declarative role-based access control (`RequireRole`) extractors | [Read README](crates/ferrox-guards/README.md) |
| | `ferrox-interceptors` | Lifecycle execution interceptors & `CacheInterceptor` pipelines | [Read README](crates/ferrox-interceptors/README.md) |
| | `ferrox-crud-gen` | Procedural macros (`crud_router!`, `vertical_slice!`) for zero-boilerplate CRUD | [Read README](crates/ferrox-crud-gen/README.md) |
| **Databases** | `ferrox-database-core` | Abstract `Repository<Entity, Id>` trait and generic persistence contracts | [Read README](crates/database/ferrox-database-core/README.md) |
| | `ferrox-database-seaorm` | Relational database ORM driver (Postgres, MySQL, SQLite) | [Read README](crates/database/ferrox-database-seaorm/README.md) |
| | `ferrox-database-mongo` | Document database driver wrapper and BSON repository | [Read README](crates/database/ferrox-database-mongo/README.md) |
| | `ferrox-database-redis` | In-memory key-value cache client, connection pool, and pub/sub | [Read README](crates/database/ferrox-database-redis/README.md) |
| | `ferrox-migrations` | Automatic database schema migration runner | [Read README](crates/ferrox-migrations/README.md) |
| **Resilience & Security**| `ferrox-security` | PASETO v4 token translation engine, JWT claims, and Argon2id hashing | [Read README](crates/ferrox-security/README.md) |
| | `ferrox-singleflight` | Cache stampede (dogpile effect) prevention using broadcast channels | [Read README](crates/ferrox-singleflight/README.md) |
| | `ferrox-circuit-breaker` | Circuit breaker pattern (`Closed`, `Open`, `HalfOpen`) state machine | [Read README](crates/ferrox-circuit-breaker/README.md) |
| | `ferrox-rate-limiter` | Redis-backed token bucket and fixed-window rate limiters | [Read README](crates/ferrox-rate-limiter/README.md) |
| | `ferrox-sync` | Distributed locking mechanisms (Redis Redlock & SQL advisory locks) | [Read README](crates/ferrox-sync/README.md) |
| **Architectures** | `ferrox-cqrs` | Decoupled `CommandBus` and `QueryBus` dispatchers | [Read README](crates/ferrox-cqrs/README.md) |
| | `ferrox-saga` | Saga orchestrator engine for multi-step distributed transactions | [Read README](crates/ferrox-saga/README.md) |
| | `ferrox-events` | Strongly-typed `DomainEvent` dispatcher & pub/sub broadcast bus | [Read README](crates/ferrox-events/README.md) |
| | `ferrox-jobs` | Async background worker queue engine powered by Redis & Apalis | [Read README](crates/ferrox-jobs/README.md) |
| | `ferrox-schedule` | Async cron job scheduler and scheduled task orchestrator | [Read README](crates/ferrox-schedule/README.md) |
| **Observability** | `ferrox-logger` | Structured JSON tracing subscriber and Sentry integration | [Read README](crates/ferrox-logger/README.md) |
| | `ferrox-health` | Kubernetes `/healthz` (liveness) and `/readyz` (readiness) probe handlers | [Read README](crates/ferrox-health/README.md) |
| | `ferrox-metrics` | Prometheus metrics exporter and latency histograms | [Read README](crates/ferrox-metrics/README.md) |
| | `ferrox-tracing` | OpenTelemetry OTLP distributed tracing & correlation ID propagation | [Read README](crates/ferrox-tracing/README.md) |
| **Transports** | `ferrox-transports` | Multi-protocol transport abstractions (HTTP, gRPC, WebSockets) | [Read README](crates/ferrox-transports/README.md) |
| | `ferrox-graphql` | GraphQL integration with `async-graphql` schema builders & SDL export | [Read README](crates/ferrox-graphql/README.md) |
| | `ferrox-sse` | Server-Sent Events (SSE) push stream response builders | [Read README](crates/ferrox-sse/README.md) |
| | `ferrox-storage` | Unified file storage abstraction (Local disk, S3, MinIO) | [Read README](crates/ferrox-storage/README.md) |
| | `ferrox-datagrid` | AG-Grid, MUI X, and TanStack Table query parameter translators | [Read README](crates/ferrox-datagrid/README.md) |
| **Integrations** | `ferrox-mailer` | Transactional email dispatcher backed by Lettre (SMTP, SendGrid, SES) | [Read README](crates/integrations/ferrox-mailer/README.md) |
| | `ferrox-notifications-slack` | Slack incoming webhook alert adapter | [Read README](crates/integrations/ferrox-notifications-slack/README.md) |
| | `ferrox-payments-stripe` | Stripe Checkout & Webhook signature verification | [Read README](crates/integrations/ferrox-payments-stripe/README.md) |
| | `ferrox-payments-google` | Google Pay & Play Store in-app purchase verification | [Read README](crates/integrations/ferrox-payments-google/README.md) |
| | `ferrox-feature-flags` | Redis-backed feature toggle evaluation engine | [Read README](crates/integrations/ferrox-feature-flags/README.md) |
| | `ferrox-webhooks` | Outgoing webhook dispatcher with HMAC signatures & backoff retry | [Read README](crates/integrations/ferrox-webhooks/README.md) |
| | `ferrox-reports` | CSV and Excel report generation utilities | [Read README](crates/integrations/ferrox-reports/README.md) |
| | `ferrox-cloud-helpers` | AWS Secrets Manager and cloud provider SDK helpers | [Read README](crates/integrations/ferrox-cloud-helpers/README.md) |
| | `ferrox-i18n` | Multi-language localization and Accept-Language header parsers | [Read README](crates/integrations/ferrox-i18n/README.md) |
| **Tooling** | `cargo-ferrox` | Command Line Interface (`cargo ferrox init`, `cargo ferrox generate`) | [Read README](crates/ferrox-cli/README.md) |

---

## 🚀 End-to-End Quick Start

### 1. Add Ferrox Dependencies to `Cargo.toml`

```toml
[dependencies]
ferrox-app = { version = "0.1.2", path = "crates/ferrox-app" }
ferrox-transports = { version = "0.1.2", path = "crates/ferrox-transports" }
ferrox-logger = { version = "0.1.2", path = "crates/ferrox-logger" }
ferrox-errors = { version = "0.1.2", path = "crates/ferrox-errors" }
ferrox-health = { version = "0.1.2", path = "crates/ferrox-health" }
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

### 2. Create Your Application (`src/main.rs`)

```rust
use axum::{routing::get, Json, Router};
use ferrox_app::FerroxApp;
use ferrox_logger::{setup_logger, LoggerConfig};
use ferrox_transports::http::HttpTransport;
use ferrox_errors::AppError;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Structured JSON Logger & Sentry Guard
    let _sentry = setup_logger(LoggerConfig::default())?;
    tracing::info!("Initializing Ferrox backend service...");

    // 2. Define Route Handlers
    let api_routes = Router::new()
        .route("/health", get(|| async { Json(json!({ "status": "UP", "framework": "Ferrox" })) }))
        .route("/users", get(list_users_handler));

    // 3. Configure HTTP Transport Layer
    let transport = HttpTransport::new(api_routes, 8080)
        .with_strict_cors(vec!["http://localhost:3000"]);

    // 4. Boot Ferrox Multi-Transport App
    FerroxApp::new()
        .add_transport(transport)
        .start()
        .await?;

    Ok(())
}

async fn list_users_handler() -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(json!([
        { "id": 1, "username": "alice", "role": "admin" },
        { "id": 2, "username": "bob", "role": "user" }
    ])))
}
```

---

## 📚 Documentation

The interactive documentation portal is built with Docusaurus and located in the [`docs/`](docs/) directory.

To run the documentation portal locally:

```bash
cd docs
npm install
npm run start
```

Visit `http://localhost:3000` to browse the interactive guides, API references, and architecture deep dives.

---

## 💬 Community & Support

Join the official global **Ferrox Community** to discuss framework architecture, ask questions, share showcases, and collaborate with developers worldwide:

[![Discord Server](https://img.shields.io/badge/Discord-Ferrox%20Community-5865F2?style=for-the-badge&logo=discord&logoColor=white)](https://discord.gg/Bx3CzGec7d)
[![Reddit Subreddit](https://img.shields.io/badge/Reddit-r%2FFerrox-FF4500?style=for-the-badge&logo=reddit&logoColor=white)](https://www.reddit.com/r/Ferrox/)
[![Email Support](https://img.shields.io/badge/Email-info%40ferrox--rust.dev-D14836?style=for-the-badge&logo=gmail&logoColor=white)](mailto:info@ferrox-rust.dev)
[![Sponsor via PayPal](https://img.shields.io/badge/Sponsor-PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.com/donate/?hosted_button_id=9Q3UG829FHT6J)

- **💖 Sponsor & Support Development:** [Donate via PayPal](https://www.paypal.com/donate/?hosted_button_id=9Q3UG829FHT6J) (Aniello Tortora - Merchant Code: `HLLNK8UNDS576`)
- **Official Support Email:** [info@ferrox-rust.dev](mailto:info@ferrox-rust.dev)
- **Discord Invite Link:** [https://discord.gg/Bx3CzGec7d](https://discord.gg/Bx3CzGec7d)
- **Reddit Subreddit:** [r/Ferrox](https://www.reddit.com/r/Ferrox/)

---

## 📜 License

Ferrox is dual-licensed under either of the following licenses at your option:

- **[MIT License](LICENSE-MIT)**
- **[Apache License, Version 2.0](LICENSE-APACHE)**

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Ferrox by you shall be dual-licensed as above, without any additional terms or conditions.
