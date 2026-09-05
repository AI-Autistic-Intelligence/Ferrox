---
sidebar_position: 1
---

# 🌐 Multi-Transport Application System

Most traditional web frameworks only support HTTP REST servers out of the box. Enterprise backend applications, however, often need to run multiple server protocols simultaneously—for example, an HTTP REST API server on port `3000`, a gRPC microservice server on port `50051`, and an FTP or WebSocket file server.

Ferrox decouples server transports from the core application using the `Transport` trait and `FerroxApp` lifecycle manager (`ferrox-transports` & `ferrox-app`).

---

## 1. The `Transport` Trait

All transport layers implement the `Transport` trait:

```rust
use async_trait::async_trait;
use ferrox_errors::AppError;

#[async_trait]
pub trait Transport: Send + Sync {
    /// Returns the name of the transport layer (e.g. "HttpTransport", "GrpcTransport")
    fn name(&self) -> &'static str;

    /// Starts the transport server asynchronously
    async fn start(&self) -> Result<(), AppError>;
}
```

---

## 2. Bootstrapping Concurrent Transports in `FerroxApp`

`FerroxApp` spawns all added transports into concurrent Tokio tasks and manages OS signals (`SIGINT`, `SIGTERM`) for graceful shutdown:

```rust
use axum::{routing::get, Router};
use ferrox_app::FerroxApp;
use ferrox_transports::http::HttpTransport;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_router = Router::new().route("/ping", get(|| async { "pong" }));

    // HTTP Transport listening on port 3000
    let http = HttpTransport::new(http_router, 3000);

    // Multi-transport execution
    FerroxApp::new()
        .add_transport(http)
        // .add_transport(grpc_transport)
        // .add_transport(ftp_transport)
        .start()
        .await?;

    Ok(())
}
```

---

## 3. Graceful Shutdown Flow

When a `Ctrl+C` or `SIGTERM` signal is received by `FerroxApp`:
1. `FerroxApp` stops accepting new incoming connections across all active transports.
2. In-flight requests are allowed to finish within a graceful timeout.
3. Database pools, Redis connections, and background workers close cleanly.
