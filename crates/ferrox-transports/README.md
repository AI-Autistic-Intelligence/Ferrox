# Ferrox Transports (`ferrox-transports`)

`ferrox-transports` defines the core `Transport` trait and protocol server builders (HTTP Axum, gRPC Tonic, WebSockets)
managed by `FerroxApp`.

## Key Features
- 🌐 **`Transport` Trait**: Common async interface (`start()`, `name()`) for all network protocol listeners.
- 🚀 **`HttpTransport` Builder**: Axum HTTP server wrapper with CORS, timeouts, and fallback routing.
