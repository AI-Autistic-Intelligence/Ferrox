---
sidebar_position: 4
---

# 🚀 Singleflight: Cache Stampede Protection

When a high-traffic cache key expires (e.g. homepage top products or hot news stories), hundreds or thousands of concurrent incoming HTTP requests simultaneously miss the cache and hit the primary database.

This phenomenon is known as the **Cache Stampede (Dogpile Effect)** and frequently crashes database clusters under peak load.

`ferrox-singleflight` eliminates cache stampedes by ensuring that **only one request executes the expensive computation**, while all other concurrent requests wait and share the result.

---

## 1. How Singleflight Works

```text
Concurrent Requests for key "hot_product_42":

Request 1  ---> [ Executes DB Query ] --------------------+---> Returns Result
                                                          |
Request 2  ---> [ Suspends & Waits for Request 1 ] -------+---> Shares Result
                                                          |
Request 3  ---> [ Suspends & Waits for Request 1 ] -------+---> Shares Result
```

Using Tokio broadcast channels and thread-safe DashMaps, `Singleflight` deduplicates identical in-flight executions with zero network or locking overhead.

---

## 2. Using `Singleflight<T>`

```rust
use ferrox_singleflight::Singleflight;
use ferrox_errors::AppError;

let group: Singleflight<ProductDto> = Singleflight::new();

// Simulate 100 concurrent incoming requests for product 42
let cache_key = "product_42";

let result = group.execute(cache_key, || async move {
    println!("🔥 SINGLEFLIGHT: Primary DB query executing for key: {}", cache_key);
    // Heavy DB query or slow computation...
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    
    Ok(ProductDto {
        id: 42,
        name: "Ferrox Pro".into(),
    })
}).await?;
```

---

## 3. Performance Impact

| Metric | Without Singleflight | With Ferrox Singleflight |
|---|---|---|
| Concurrent DB Queries (1,000 requests) | 1,000 Queries | **1 Query** |
| Database CPU Load Spike | 100% (Crash Risk) | **Normal (&lt;2%)** |
| Response Latency for Waiters | High (Queue Bottleneck) | **Identical to Request 1** |

---

## 4. ✅ Best Practices

- **Wrap all hot cache fallback functions in `Singleflight`**: Whenever querying a database after a cache miss, wrap the lookup in `Singleflight` to guarantee zero database dogpiling.
