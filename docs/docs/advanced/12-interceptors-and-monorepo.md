---
sidebar_position: 12
---

# Global Interceptors & Monorepo CLI

`Ferrox` provides high-level abstractions common in enterprise frameworks like NestJS, empowering you to control the lifecycle of requests and easily manage large-scale architectures.

## 1. Global Interceptors (`ferrox-interceptors`)

Interceptors allow you to bind logic that executes *around* your route handlers. This is essential for:
- Measuring request performance (execution time).
- Standardizing error formats globally.
- Transforming response payloads.

`Ferrox` includes a built-in `logging_interceptor` that wraps the Axum request lifecycle to measure execution time.

```rust
use axum::Router;
use axum::middleware::from_fn;
use ferrox_interceptors::logging_interceptor;

let app = Router::new()
    // ... routes
    .layer(from_fn(logging_interceptor));
```

When active, it outputs exact millisecond execution times for every route:
`[200] GET /api/users - 12.4ms`

## 2. Monorepo Scaffolding (`ferrox-cli`)

An enterprise application rarely lives in a single folder. It's usually a Monorepo containing an API Gateway, Microservices, and Shared DTOs.

The `Ferrox CLI` automates this architectural heavy lifting. By running `ferrox init` (or `cargo run -p ferrox-cli -- init`), the wizard will interactively ask you for your preferences and then physically scaffold a full Cargo Workspace Monorepo:

```text
/my-new-enterprise-app
  ├── /apps
  │    ├── /api-gateway
  │    └── /microservice-auth
  ├── /packages
  │    ├── /shared-dto
  │    └── /database
  ├── docker-compose.yml
  └── Cargo.toml (Workspace)
```

This ensures that your team starts from Day 1 with a scalable, decoupled architecture instead of a monolithic tightly-coupled application.
