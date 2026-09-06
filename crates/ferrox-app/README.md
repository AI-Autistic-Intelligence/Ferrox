# Ferrox App (`ferrox-app`)

`ferrox-app` provides the primary application bootstrapper and multi-transport lifecycle orchestrator
for the Ferrox framework. It manages concurrent server instances (HTTP, gRPC, WebSockets, etc.) and enforces
graceful shutdown handling across UNIX signals (`SIGTERM`) and cross-platform interrupts (`Ctrl+C`).

## Architectural Role
In enterprise applications, backends often need to serve multiple network protocols simultaneously (e.g. Axum for HTTP/REST,
Tonic for gRPC inter-service communication). `FerroxApp` encapsulates these network transports into unified `Arc<dyn Transport>`
workers and manages their startup, execution lifecycle, and teardown concurrently.

## Key Features
- 🌐 **Multi-Transport Execution**: Boot HTTP, gRPC, and background listeners concurrently.
- 🛡️ **Graceful Shutdown Orchestration**: Catches OS termination signals and shuts down active transport threads cleanly.
- ⚡ **Integration with Tower & Sentry**: Built-in support for middleware, cors, timeouts, and error capturing.

## Example Usage
```rust,no_run
use ferrox_app::FerroxApp;
use ferrox_transports::http::HttpTransport;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let router = Router::new().route("/health", get(|| async { "OK" }));
    let transport = HttpTransport::new(router, 8080);

    FerroxApp::new()
        .add_transport(transport)
        .start()
        .await?;

    Ok(())
}
```
