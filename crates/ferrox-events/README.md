# Ferrox Events (`ferrox-events`)

`ferrox-events` provides an event dispatcher and pub/sub message bus for building decoupled, event-driven backends in Rust.
It allows application components to publish strongly-typed `DomainEvent` instances without direct coupling between services.

## Design Rationale
In modular microservice and monolithic architectures, side effects (such as sending a welcome email when a user registers)
should be executed asynchronously without clogging the main HTTP request flow. `ferrox-events` provides an in-memory broadcast
dispatcher with asynchronous event subscribers.

## Key Features
- 📢 **`DomainEvent` Trait**: Define custom payload events with metadata and timestamps.
- 📻 **`InMemoryDispatcher`**: Non-blocking async event broadcasting using Tokio channels.
- 🔌 **Extensible Backends**: Easy adapter bridge for Redis Pub/Sub, NATS, or RabbitMQ.
