---
sidebar_position: 3
---

# ⚡ Circuit Breakers & Fault Tolerance

In distributed microservices, downstream database outages or third-party API latency spikes can exhaust server connection threads, causing **cascading failures** across your entire infrastructure.

`ferrox-circuit-breaker` isolates unstable external dependencies by wrapping calls in a `CircuitBreaker` state machine.

---

## 1. The Circuit Breaker State Machine

A Circuit Breaker operates in 3 distinct states:

```
        +---------------------------------------------------+
        |                                                   |
        v                                                   |
  +-----------+    Failure Threshold Reached    +----------+
  |  CLOSED   | ------------------------------> |   OPEN   |
  | (Normal)  |                                 | (Blocked)|
  +-----------+                                 +----------+
        ^                                            |
        |               Reset Timeout Passed         |
        |              +-------------------+         |
        |              |                   |         |
        +------- Success                   v         v
                +------------------------------------+
                |             HALF-OPEN              |
                |          (Trial Request)           |
                +------------------------------------+
```

1. **Closed**: Normal state. All requests pass through. Failures are counted.
2. **Open**: Failure threshold reached. All requests are immediately rejected without calling the downstream dependency.
3. **Half-Open**: Reset timeout expires. A trial request is permitted. If successful, circuit resets to **Closed**; if failed, returns to **Open**.

---

## 2. Using `CircuitBreaker`

```rust
use std::time::Duration;
use ferrox_circuit_breaker::CircuitBreaker;
use ferrox_errors::AppError;

// Create circuit breaker: trips to OPEN after 5 consecutive failures, resets after 30 seconds
let breaker = CircuitBreaker::new(5, Duration::from_secs(30));

let result = breaker.execute(|| async {
    // Call downstream HTTP microservice or third-party API
    reqwest::get("https://api.stripe.com/v1/charges").await
        .map_err(|e| AppError::InternalServerError(Box::new(e)))
}).await;

match result {
    Ok(response) => println!("Success: {:?}", response),
    Err(AppError::InternalServerError(msg)) if msg.to_string().contains("Circuit Open") => {
        println!("⚠️ Request blocked: Downstream service is currently unstable!");
    }
    Err(e) => println!("API Error: {:?}", e),
}
```

---

## 3. Benefits of Circuit Breakers

- **Prevent Resource Starvation**: Stop Tokio worker threads from waiting on timing-out third-party APIs.
- **Fast Failures**: Instantly respond to users when external services are down instead of hanging for 30 seconds.
- **Graceful Recovery**: Allow failing microservices time to recover without slamming them with traffic.
