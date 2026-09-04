---
sidebar_position: 3
---

# Redis

Redis is the core of our caching layer and is deeply integrated into Rust-FERROX via the `ferrox-database-redis` module.

## Setup

The Redis manager leverages `deadpool_redis` to maintain an asynchronous connection pool.

```rust
use ferrox_database_redis::RedisManager;

let redis_pool = RedisManager::new().await;
```

## Ecosystem Synergy
Redis powers multiple sub-crates in Rust-FERROX:
1. **ferrox-jobs**: Serves as the message broker for Apalis.
2. **ferrox-rate-limiter**: Stores IP tracking info.
3. **ferrox-feature-flags**: Centralized toggle states for instant propagation across all pods.
