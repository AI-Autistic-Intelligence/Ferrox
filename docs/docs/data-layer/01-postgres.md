---
sidebar_position: 1
---

# PostgreSQL (SeaORM)

Rust-YALC uses **SeaORM** as the primary ORM for relational databases, encapsulated within the `yalc-database-seaorm` crate.

## Why SeaORM?
- Fully Async
- Dynamic query building
- Excellent integration with SQLx for connection pooling

## Initializing the Connection

The workspace configures a highly optimized connection pool that immediately fails (Fail-Fast) if the database is unreachable, avoiding ghost starts.

```rust
use yalc_database_seaorm::db::Database;

// Connects using POSTGRES_URL from env
let db_conn = Database::connect().await.expect("Failed to connect to PostgreSQL");
```

## Creating Entities

You can generate entities automatically from your schema using `sea-orm-cli` or define them manually in `crates/yalc-types/`.
