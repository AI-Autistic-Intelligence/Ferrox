# Ferrox Health (`ferrox-health`)

`ferrox-health` provides Kubernetes-compliant `/healthz` (liveness) and `/readyz` (readiness) health check endpoints for Ferrox backends.

## Key Features
- 🩺 **Liveness Probe**: Fast endpoint indicating that the service process is active.
- 🚦 **Readiness Probe**: Performs dynamic checks against database pools, Redis, and external dependencies before routing traffic.
