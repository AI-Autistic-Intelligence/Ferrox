---
sidebar_position: 1
---

# ⚡ High-Throughput Benchmarks & Tuning

Ferrox is designed to process **100,000+ HTTP requests per second** per node with sub-millisecond latencies.

Achieving peak hardware utilization under extreme concurrency requires fine-tuning Tokio runtime worker threads, choosing high-performance memory allocators (`jemalloc`/`mimalloc`), and tuning TCP socket parameters.

---

## 1. Memory Allocators (`jemalloc` / `mimalloc`)

The standard C library allocator (`glibc malloc`) suffers from severe lock contention and memory fragmentation when running hundreds of concurrent Tokio worker threads allocating and freeing short-lived JSON buffers.

Replacing the global memory allocator with `jemalloc` or `mimalloc` yields a **20-35% throughput increase**:

### Integrating `tikv-jemallocator`

Add `tikv-jemallocator` to `Cargo.toml`:

```toml
[dependencies]
tikv-jemallocator = "0.5"
```

Configure `jemalloc` as global allocator in `main.rs`:

```rust
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
```

---

## 2. Tuning the Tokio Runtime

By default, Tokio spawns one worker thread per CPU core. For I/O-bound microservices handling 50k+ active WebSockets or HTTP connections, configure custom runtime thread counts:

```rust
fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(16)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // Run FerroxApp...
        });
}
```

---

## 3. Database Connection Pool Sizing Formula

Setting database pool max connections too high causes CPU context-switching churn on DB nodes.

Use the standard PostgreSQL connection pool formula:

```text
Max Connections = (CPU Cores * 2) + Effective Spindle Count
```

For a 4-core database server with SSD storage:
```text
Max Connections = (4 * 2) + 1 = 9 Connections
```

---

## 4. Benchmark Comparison

Benchmark executed using `wrk -t12 -c400 -d30s http://127.0.0.1:3000/api/v1/ping`:

| Framework | Runtime Engine | Throughput (req/sec) | P99 Latency | Memory Footprint |
|---|---|---|---|---|
| NestJS (Node.js 20) | Express / V8 | 14,200 req/sec | 28.4 ms | 185 MB |
| Spring Boot 3 | Java 21 / Netty | 38,500 req/sec | 12.1 ms | 340 MB |
| **Ferrox Framework** | **Tokio / Axum** | **118,400 req/sec** | **0.84 ms** | **14 MB** |
