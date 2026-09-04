---
sidebar_position: 10
---

# Zero Trust Infrastructure (Least Privilege)

Building a secure Framework isn't just about API tokens; it's about the entire infrastructure. `Rust-FERROX` implements a "Deny by Default" mentality across databases and networks.

## 1. Database Least Privilege

Connecting your application to Postgres or MongoDB using the `root` user is a critical vulnerability. If an attacker finds a SQL injection flaw, they can drop your entire cluster.

`Rust-FERROX` provides standard initialization scripts in `scripts/security/`:
- **Postgres (`init_postgres_least_privilege.sql`)**: Revokes public permissions and creates an application user that can only perform CRUD operations on the specific `ferrox_db` tables. It cannot create or drop databases or roles.
- **MongoDB (`init_mongo_least_privilege.js`)**: Creates a user restricted with the `readWrite` role exclusively on the `ferrox_db` database.

These scripts are automatically injected into the `docker-compose.yml` volumes (`/docker-entrypoint-initdb.d/`). When you start your local environment, your databases are born secure.

## 2. Strict CORS Management

By default, an HTTP server might accept requests from any origin, leading to Cross-Origin attacks.

`Rust-FERROX` exposes a fluent builder in the `HttpTransport` to enforce explicit domain allowlisting:

```rust
use ferrox_transports::HttpTransport;
use axum::Router;

let router = Router::new(); // Your routes

let http_layer = HttpTransport::new(router, 3000)
    // Enforces Zero Trust CORS: Deny everything EXCEPT these domains
    .with_strict_cors(vec![
        "https://my-frontend.com",
        "https://admin.my-frontend.com"
    ]);

// Pass to FerroxApp
```

This prevents developers from accidentally exposing the API to the public web while trying to fix local CORS errors.
