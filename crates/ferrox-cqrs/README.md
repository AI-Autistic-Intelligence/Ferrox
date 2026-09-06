# Ferrox CQRS (`ferrox-cqrs`)

`ferrox-cqrs` implements the Command Query Responsibility Segregation (CQRS) pattern for Ferrox backend services,
splitting write operations (`CommandBus`) from read operations (`QueryBus`).

## Key Features
- ✉️ **`CommandBus` & `QueryBus`**: Decouple request handlers from domain business logic handlers.
- ⚡ **Type-Safe Dispatching**: Async handler mapping guaranteed by Rust trait bounds.
