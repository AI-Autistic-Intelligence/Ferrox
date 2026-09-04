---
sidebar_position: 2
---

# Kubernetes

Rust-YALC is designed to run in highly available environments like Kubernetes. 

## Probes

Since Rust-YALC is built with a "Fail-Fast" design, it correctly implements Liveness and Readiness probes.

```yaml
livenessProbe:
  httpGet:
    path: /health/liveness
    port: 8080
  initialDelaySeconds: 5
  periodSeconds: 10

readinessProbe:
  httpGet:
    path: /health/readiness
    port: 8080
  initialDelaySeconds: 5
  periodSeconds: 10
```

The readiness probe ensures that the pod doesn't receive traffic if a critical database (PostgreSQL, Redis, or MongoDB) becomes unreachable or responds with high latency.
