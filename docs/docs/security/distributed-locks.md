---
sidebar_position: 5
---

# 🔒 Distributed Synchronization & Locks

In multi-instance microservice deployments, running background processes or mutating shared resources (e.g., inventory deduction or batch invoice generation) requires cross-process synchronization to prevent race conditions.

`ferrox-sync` provides distributed locking abstractions backed by Redis and in-memory synchronizers.

---

## 1. Using Distributed Locks

`ferrox-sync` allows microservices to acquire mutual exclusion locks over Redis:

```rust
use ferrox_sync::DistributedLock;
use std::time::Duration;

let lock = DistributedLock::new("redis://127.0.0.1:6379");

// Attempt to acquire lock for resource "inventory_update_101" with a 10-second TTL
if let Ok(guard) = lock.acquire("inventory_update_101", Duration::from_secs(10)).await {
    println!("🔒 Lock acquired! Safely mutating shared inventory...");
    
    // Perform critical section logic...

    // Lock automatically releases when `guard` drops!
} else {
    println!("⚠️ Resource is locked by another instance. Skipping operation.");
}
```

---

## 2. Redlock Algorithm

For high-availability clusters, `ferrox-sync` implements the **Redlock Algorithm**, acquiring consensus across multiple independent Redis master nodes to ensure safety even if individual Redis nodes fail.

---

## 3. ✅ Best Practices

- **Always configure short TTLs on locks**: Set lock auto-release timeouts to prevent deadlocks if a microservice process crashes unexpectedly during critical section execution.
