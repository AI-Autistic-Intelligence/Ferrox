---
sidebar_position: 3
---

# 📈 Prometheus Metrics Export

Monitoring real-time application throughput, request latencies, and error rates requires exporting metrics to monitoring platforms like Prometheus and Grafana.

`ferrox-metrics` initializes a Prometheus HTTP exporter running on a dedicated telemetry listener port.

---

## 1. Initializing Prometheus Exporter

Call `setup_metrics` during application bootstrap:

```rust
use ferrox_metrics::{setup_metrics, record_http_request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Starts Prometheus HTTP listener on port 9000
    setup_metrics()?;

    println!("📊 Prometheus metrics available at http://0.0.0.0:9000/metrics");
    Ok(())
}
```

---

## 2. Pre-Registered Metric Counters

`ferrox-metrics` pre-registers standard counters:

| Metric Name | Type | Description |
|---|---|---|
| `http_requests_total` | Counter | Total number of HTTP requests processed |
| `db_queries_total` | Counter | Total number of database queries executed |
| `events_published_total` | Counter | Total number of domain events published |

---

## 3. Incrementing Counters in Controllers

Increment counters using helper functions or standard `metrics` macros:

```rust
use ferrox_metrics::record_http_request;
use metrics::counter;

pub async fn my_controller_handler() {
    record_http_request();
    counter!("orders_created_total").increment(1);
}
```

---

## 4. Scraping `/metrics` Output

```bash
curl http://localhost:9000/metrics
```

Output:
```text
# HELP http_requests_total Total number of HTTP requests processed
# TYPE http_requests_total counter
http_requests_total 15420
```
