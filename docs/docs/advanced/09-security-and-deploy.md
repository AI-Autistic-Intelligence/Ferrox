---
sidebar_position: 9
---

# Zero Trust Security & Deployment

`Rust-YALC` implements an uncompromising Zero Trust security model via the `yalc-security` crate, paired with production-ready Deployment infrastructure.

## 1. Zero Trust Authentication
Using JSON Web Tokens (JWT) is no longer the industry standard due to numerous cryptographic pitfalls. `Rust-YALC` uses **PASETO (Platform-Agnostic Security Tokens)** to guarantee token integrity.

### The Axum Auth Middleware
We provide a plug-and-play middleware (`require_auth`) that intercepts every request, cryptographically validates the token, and injects the user data into the request context. If validation fails, it aborts the request immediately with `401 Unauthorized`.

```rust
use axum::{routing::get, Router};
use axum::middleware::from_fn_with_state;
use yalc_security::auth_middleware::require_auth;

// Protect an entire router
let protected_router = Router::new()
    .route("/profile", get(get_profile))
    .layer(from_fn_with_state(auth_engine.clone(), require_auth));
```

## 2. Role-Based Access Control (RBAC)
You can protect specific endpoints based on the user's role using the `require_role` guard:

```rust
use yalc_security::auth_middleware::require_role;

// Only Admins can delete users
.route("/users/:id", delete(delete_user)
    .route_layer(axum::middleware::from_fn(|req, next| require_role(req, next, "admin")))
)
```

## 3. Production Deployment
`Rust-YALC` ships with everything you need to deploy globally.

### Docker (Cargo Chef)
The root `Dockerfile` uses a multi-stage build powered by `cargo-chef`. This aggressively caches dependencies, reducing rebuild times in CI/CD from minutes to seconds.
The final image uses `debian-buster-slim` and drops root privileges (`USER yalc`) for maximum security.

### Docker Compose
The `docker-compose.yml` provides a one-click infrastructure bootstrap:
```bash
docker-compose up -d
```
It spins up:
- Postgres (SeaORM)
- MongoDB
- Redis (Caching/Jobs)
- Meilisearch (Lexical Search)
- Qdrant (AI Vector Search)

### CI/CD
A standard GitHub Actions pipeline (`.github/workflows/ci.yml`) is included to automatically enforce formatting, linting (`clippy`), and run unit tests on every push.
