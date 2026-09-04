---
sidebar_position: 3
---

# Redis

Redis is the core of our caching layer and is deeply integrated into Rust-YALC via the `yalc-database-redis` module.

## Setup

The Redis manager leverages `deadpool_redis` to maintain an asynchronous connection pool.

```rust
use yalc_database_redis::RedisManager;

let redis_pool = RedisManager::new().await;
```

## Ecosystem Synergy
Redis powers multiple sub-crates in Rust-YALC:
1. **yalc-jobs**: Serves as the message broker for Apalis.
2. **yalc-rate-limiter**: Stores IP tracking info.
3. **yalc-feature-flags**: Centralized toggle states for instant propagation across all pods.
