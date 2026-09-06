//! # Ferrox Saga (`ferrox-saga`)
//!
//! `ferrox-saga` implements a Saga Orchestrator engine for managing multi-step distributed transactions across microservices
//! with automatic rollback compensation handling upon failure.
//!
//! ## Rationale
//! In microservices architectures, traditional two-phase commit (2PC) database transactions do not scale across network boundaries.
//! The Saga pattern breaks complex business operations into a sequence of steps, where each step has an associated compensating action
//! triggered on failure.
//!
//! ## Key Features
//! - 🔄 **`SagaBuilder`**: Declarative registration of transaction steps and compensating functions.
//! - 🛡️ **Failure Compensation**: Executes rollback steps in reverse order when any transaction step fails.

use async_trait::async_trait;
use tracing::{info, error, warn};
use ferrox_errors::AppError;
use std::sync::Arc;

/// Represents a single step in a Saga.
/// A step must provide both the forward action (`execute`) and the compensating rollback (`compensate`).
#[async_trait]
pub trait SagaStep<State>: Send + Sync {
    /// The name of this step, useful for logging and observability.
    fn name(&self) -> &'static str;

    /// The forward action to execute.
    async fn execute(&self, state: &mut State) -> Result<(), AppError>;

    /// The compensating action to execute if any subsequent step fails.
    /// This should attempt to undo whatever `execute` did.
    async fn compensate(&self, state: &mut State) -> Result<(), AppError>;
}

/// The Orchestrator engine that manages a Saga's execution flow.
pub struct SagaOrchestrator<State> {
    steps: Vec<Arc<dyn SagaStep<State>>>,
    state: State,
}

impl<State> SagaOrchestrator<State> {
    /// Creates a new Orchestrator with an initial state context.
    pub fn new(state: State) -> Self {
        Self {
            steps: Vec::new(),
            state,
        }
    }

    /// Adds a step to the Saga chain.
    pub fn add_step<T: SagaStep<State> + 'static>(mut self, step: T) -> Self {
        self.steps.push(Arc::new(step));
        self
    }

    /// Executes the saga sequentially.
    /// If a step fails, it stops execution and triggers compensations in reverse order.
    pub async fn execute(mut self) -> Result<State, AppError> {
        let mut executed_steps = Vec::new();

        for step in &self.steps {
            info!("Saga Executing Step: {}", step.name());
            
            match step.execute(&mut self.state).await {
                Ok(_) => {
                    // Push to the stack of successfully executed steps
                    executed_steps.push(Arc::clone(step));
                }
                Err(e) => {
                    error!("Saga Step '{}' failed: {:?}", step.name(), e);
                    warn!("Initiating Saga Compensation (Rollback)...");

                    // Trigger compensations in reverse order (LIFO)
                    while let Some(completed_step) = executed_steps.pop() {
                        warn!("Saga Compensating Step: {}", completed_step.name());
                        
                        if let Err(comp_err) = completed_step.compensate(&mut self.state).await {
                            // If a compensation fails, we have a critical inconsistency.
                            // In a real distributed system, this requires manual intervention or a dead-letter queue.
                            error!("CRITICAL: Compensation failed for '{}': {:?}", completed_step.name(), comp_err);
                        }
                    }

                    // Return the original error that caused the saga to fail
                    return Err(e);
                }
            }
        }

        info!("Saga executed successfully.");
        Ok(self.state)
    }
}

pub fn setup() {
    println!("ferrox-saga initialized: Saga Orchestrator engine is ready.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};

    struct OrderState {
        pub balance: i32,
        pub inventory: i32,
        pub order_status: String,
    }

    struct ReserveInventoryStep;
    #[async_trait]
    impl SagaStep<OrderState> for ReserveInventoryStep {
        fn name(&self) -> &'static str { "ReserveInventory" }
        
        async fn execute(&self, state: &mut OrderState) -> Result<(), AppError> {
            state.inventory -= 1;
            Ok(())
        }
        
        async fn compensate(&self, state: &mut OrderState) -> Result<(), AppError> {
            state.inventory += 1;
            Ok(())
        }
    }

    struct ProcessPaymentStep;
    #[async_trait]
    impl SagaStep<OrderState> for ProcessPaymentStep {
        fn name(&self) -> &'static str { "ProcessPayment" }
        
        async fn execute(&self, state: &mut OrderState) -> Result<(), AppError> {
            if state.balance < 100 {
                return Err(AppError::BadRequest("Insufficient funds".into()));
            }
            state.balance -= 100;
            Ok(())
        }
        
        async fn compensate(&self, state: &mut OrderState) -> Result<(), AppError> {
            state.balance += 100;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_saga_successful_execution() {
        let initial_state = OrderState { balance: 150, inventory: 10, order_status: "PENDING".into() };
        
        let saga = SagaOrchestrator::new(initial_state)
            .add_step(ReserveInventoryStep)
            .add_step(ProcessPaymentStep);
            
        let result = saga.execute().await;
        
        assert!(result.is_ok());
        let final_state = result.unwrap();
        assert_eq!(final_state.inventory, 9);
        assert_eq!(final_state.balance, 50);
    }

    #[tokio::test]
    async fn test_saga_rollback_on_failure() {
        // Balance is 50, so ProcessPayment will fail!
        let initial_state = OrderState { balance: 50, inventory: 10, order_status: "PENDING".into() };
        
        let saga = SagaOrchestrator::new(initial_state)
            .add_step(ReserveInventoryStep)
            .add_step(ProcessPaymentStep);
            
        let result = saga.execute().await;
        
        assert!(result.is_err());
        
        // Wait, how do we assert the rollback state? 
        // The execute() consumes the Orchestrator and returns the error, destroying the state.
        // Let's modify the engine to maybe return (State, Error) on failure if we want to inspect it.
        // For testing purposes, we can use an external shared state (Arc<Mutex> or atomic).
    }

    // Let's rewrite the rollback test using external atomics to prove compensation ran!
    struct AtomicState {
        inventory: Arc<AtomicI32>,
    }

    struct AtomicReserveStep {
        inventory: Arc<AtomicI32>,
    }
    
    #[async_trait]
    impl SagaStep<AtomicState> for AtomicReserveStep {
        fn name(&self) -> &'static str { "AtomicReserve" }
        async fn execute(&self, _state: &mut AtomicState) -> Result<(), AppError> {
            self.inventory.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        }
        async fn compensate(&self, _state: &mut AtomicState) -> Result<(), AppError> {
            self.inventory.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    struct FailingPaymentStep;
    
    #[async_trait]
    impl SagaStep<AtomicState> for FailingPaymentStep {
        fn name(&self) -> &'static str { "FailingPayment" }
        async fn execute(&self, _state: &mut AtomicState) -> Result<(), AppError> {
            Err(AppError::BadRequest("Insufficient Funds".into()))
        }
        async fn compensate(&self, _state: &mut AtomicState) -> Result<(), AppError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_saga_atomic_rollback() {
        let inventory = Arc::new(AtomicI32::new(10));
        let state = AtomicState { inventory: Arc::clone(&inventory) };
        
        let saga = SagaOrchestrator::new(state)
            .add_step(AtomicReserveStep { inventory: Arc::clone(&inventory) })
            .add_step(FailingPaymentStep);
            
        let result = saga.execute().await;
        
        assert!(result.is_err());
        
        // Execute subtracted 1, but compensate should have added it back!
        assert_eq!(inventory.load(Ordering::SeqCst), 10);
    }
}