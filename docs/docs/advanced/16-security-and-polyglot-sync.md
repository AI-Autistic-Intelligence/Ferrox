---
sidebar_position: 16
---

# Dual-Token Security & Polyglot Database Sync

Ferrox implements enterprise-grade patterns for securing User Sessions and maintaining data consistency across different database technologies (e.g. MongoDB to PostgreSQL).

## 1. Dual-Token Security & UUID Masking

Never expose your internal database Primary Keys (`UUIDv7` or integers) to the frontend. Doing so allows attackers to enumerate your users or guess database sizes.

`Ferrox` provides `PublicId`:
```rust
use ferrox_security::public_id::PublicId;
use uuid::Uuid;

let db_uuid = Uuid::now_v7();
// Mask it before sending to the frontend
let safe_id = PublicId::mask_uuid("usr", db_uuid);
// Output: usr_0191c...
```

For authentication, we use the `DualTokenManager` which generates:
1. **Access Token (15 min)**: Passed in the `Authorization: Bearer` header. Short lifespan minimizes the risk if stolen.
2. **Refresh Token (30 days)**: Opaque token stored in an HTTPOnly cookie or secure storage. Used to request a new Access Token.

```rust
use ferrox_security::dual_token::DualTokenManager;

let auth = DualTokenManager::new("your_secret_key");
let tokens = auth.generate_tokens("usr_0191c...").unwrap();

println!("Access Token: {}", tokens.access_token);
println!("Refresh Token: {}", tokens.refresh_token);
```

## 2. Polyglot Database Sync (`ferrox-sync`)

In CQRS or Polyglot Persistence architectures, you might write to MongoDB for speed, but need to sync that data to PostgreSQL for Data Analytics.

`ferrox-sync` handles this asynchronously using **Redis Pub/Sub (or Streams)**:

```rust
use ferrox_sync::{SyncEngine, SyncEvent, SyncMap};
use std::sync::Arc;

let engine = SyncEngine::new(redis_client);

// 1. Controller writes to MongoDB
let user = insert_into_mongo().await;

// 2. Publish Sync Event in background (Non-blocking)
let event = SyncEvent {
    source_db: "MongoDB".into(),
    target_db: "PostgreSQL".into(),
    collection: "Users".into(),
    operation: "INSERT".into(),
    payload: user,
};
engine.publish_event("sync:mongo_to_pg", event).await.unwrap();

// 3. A background worker picks it up and applies a `SyncMap`
// to transform the NoSQL JSON into an SQL Relational insert.
```
This guarantees Eventual Consistency across multiple databases without slowing down your API endpoints.
