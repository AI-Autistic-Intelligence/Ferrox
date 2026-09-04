---
sidebar_position: 2
---

# Background Jobs

When building web applications, you should avoid performing long-running tasks inside the HTTP request cycle. 
For example, generating a PDF or sending 100 emails can take seconds.

## Enter Apalis & Redis

Rust-FERROX uses **Apalis** backed by **Redis** via the `ferrox-jobs` module.

### How it works

1. **The Enqueue**: In your HTTP handler, you push a serialized job to the Redis queue.
2. **The Worker**: In the `main.rs`, we spawn a background worker that constantly polls Redis.
3. **The Execution**: The worker pulls the job and runs the async function. If it fails, it uses exponential backoff to retry.

```rust
// Spawning the worker inside main.rs
use ferrox_jobs::start_worker;

start_worker("redis://127.0.0.1:6379").await.unwrap();
```
