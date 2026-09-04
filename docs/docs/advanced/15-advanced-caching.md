---
sidebar_position: 15
---

# Advanced Enterprise Caching

Caching is not just about dumping data into Redis. High-traffic and multi-tenant applications suffer from two critical caching flaws: **Cache Stampedes** and **Data Leakage**. `Ferrox` solves both natively.

## 1. Stampede Prevention (Singleflight)

A **Cache Stampede** (or dogpile effect) happens when a popular cache key expires and 10,000 users request it at the exact same millisecond. They all miss the cache, and they all hit your database simultaneously, bringing your backend down instantly.

`Ferrox` solves this using the `ferrox-singleflight` crate. 
When multiple identical requests arrive:
1. The **first** request executes the controller and queries the database.
2. The remaining 9,999 requests are **suspended in memory** (awaiting).
3. Once the first request finishes, its result is **broadcasted** to all 9,999 waiting requests instantly, and the result is saved to Redis.
4. Your database executes **only 1 query** instead of 10,000.

## 2. Secure Cache Policies (`PrivateCache`)

Never cache sensitive data by URL alone (e.g. `GET /api/profile`). Doing so would serve User A's profile to User B!

`Ferrox` provides `CachePolicy` to guarantee data isolation:

```rust
use ferrox_interceptors::cache::{CachePolicy, CacheConfig};

// 1. Public Cache (Safe for everyone, Key = URL)
let public_config = CacheConfig {
    policy: CachePolicy::PublicCache,
    // ...
};

// 2. Private Cache (Isolated per User)
// Ferrox intercepts the JWT Bearer token, validates it using `ferrox-security`,
// extracts the User ID (sub), and prefixes the Redis key.
// Key = cache:private:{user_id}:/api/profile
let private_config = CacheConfig {
    policy: CachePolicy::PrivateCache,
    auth_secret: "your_jwt_secret".into(),
    // ...
};
```

By leveraging `PrivateCache`, you can aggressively cache sensitive user data (profiles, balances, private dashboards) in Redis with mathematical certainty that users will never see each other's data.
