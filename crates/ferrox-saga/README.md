# Ferrox Saga (`ferrox-saga`)

`ferrox-saga` implements a Saga Orchestrator engine for managing multi-step distributed transactions across microservices
with automatic rollback compensation handling upon failure.

## Rationale
In microservices architectures, traditional two-phase commit (2PC) database transactions do not scale across network boundaries.
The Saga pattern breaks complex business operations into a sequence of steps, where each step has an associated compensating action
triggered on failure.

## Key Features
- 🔄 **`SagaBuilder`**: Declarative registration of transaction steps and compensating functions.
- 🛡️ **Failure Compensation**: Executes rollback steps in reverse order when any transaction step fails.
