---
sidebar_position: 6
---

# Saga Orchestrator (yalc-saga)

When building Microservices, you can no longer rely on single-database ACID transactions (`BEGIN ... COMMIT`) to ensure data consistency across multiple services. Instead, we use the **Saga Pattern**.

Rust-YALC includes a fully-featured **Saga Orchestrator** engine (`yalc-saga`) that coordinates distributed transactions and automatically triggers Compensating Transactions (rollbacks) if a step fails.

## Core Concepts

### The `SagaStep` Trait
Every action in your distributed transaction must implement the `SagaStep` trait, providing both a forward action and a rollback action:

```rust
use yalc_saga::SagaStep;
use async_trait::async_trait;

struct ReserveInventoryStep;

#[async_trait]
impl SagaStep<OrderState> for ReserveInventoryStep {
    fn name(&self) -> &'static str { "ReserveInventory" }
    
    // Forward Action
    async fn execute(&self, state: &mut OrderState) -> Result<(), AppError> {
        // SQL to subtract inventory
        Ok(())
    }
    
    // Compensating Action (Rollback)
    async fn compensate(&self, state: &mut OrderState) -> Result<(), AppError> {
        // SQL to add inventory back
        Ok(())
    }
}
```

### The Orchestrator Engine
You build your transaction flow by passing the steps to the `SagaOrchestrator`:

```rust
use yalc_saga::SagaOrchestrator;

let saga = SagaOrchestrator::new(initial_state)
    .add_step(CreateOrderStep)
    .add_step(ReserveInventoryStep)
    .add_step(ProcessPaymentStep);
    
let result = saga.execute().await;
```

## How Rollbacks Work
If `ProcessPaymentStep.execute()` fails (e.g. Insufficient Funds):
1. The Orchestrator halts execution immediately.
2. It looks at the stack of *successfully completed* steps.
3. It pops them in **reverse order** (LIFO) and calls `.compensate()`.
4. It calls `ReserveInventoryStep.compensate()`.
5. It calls `CreateOrderStep.compensate()`.

Your system is left in a perfectly consistent state without any orphaned records or phantom inventory deductions!
