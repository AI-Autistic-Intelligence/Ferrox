# Ferrox Circuit Breaker (`ferrox-circuit-breaker`)

`ferrox-circuit-breaker` provides a circuit breaker state machine (Closed, Open, Half-Open) to prevent cascading failures
when calling external REST or gRPC services.

## Key Features
- ⚡ **State Machine**: Automatically trips to `Open` state upon reaching error thresholds, preventing downstream overload.
- ⏳ **Recovery Reset**: Transitions to `Half-Open` after a configurable cooldown period to test endpoint recovery.
