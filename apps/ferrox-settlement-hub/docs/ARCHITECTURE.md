# ferrox-settlement-hub Architecture Specification (Hexagonal DDD)

## 1. Domain Boundary & Invariants
This service adheres to the pure Domain-Driven Design (DDD) model:
- **Pure Domain (`src/domain/`)**: Zero framework or database dependencies.
- **Application (`src/application/`)**: CQRS Commands and Queries.
- **Infrastructure (`src/infrastructure/`)**: Axum controllers, adapters, repositories.

## 2. Invariant Rules
1. Order quantities must be strictly positive ($> 0$).
2. Transitions are monotonic and validated against internal state enums.
3. All domain events are emitted asynchronously via CQRS ports.
