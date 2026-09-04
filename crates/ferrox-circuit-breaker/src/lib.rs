use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Instant, Duration};
use tokio::sync::Mutex;
use tracing::{warn, info};
use ferrox_errors::AppError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CircuitState {
    Closed, // Normal operation
    Open,   // Failing, blocking requests
    HalfOpen, // Testing if the service is back
}

pub struct CircuitBreaker {
    state: Mutex<CircuitState>,
    failure_threshold: u32,
    failure_count: AtomicU32,
    reset_timeout: Duration,
    last_failure: Mutex<Option<Instant>>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, reset_timeout: Duration) -> Self {
        Self {
            state: Mutex::new(CircuitState::Closed),
            failure_threshold,
            failure_count: AtomicU32::new(0),
            reset_timeout,
            last_failure: Mutex::new(None),
        }
    }

    pub async fn execute<F, Fut, T>(&self, action: F) -> Result<T, AppError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, AppError>>,
    {
        let mut state = self.state.lock().await;

        if *state == CircuitState::Open {
            let last_fail = self.last_failure.lock().await.unwrap();
            if last_fail.elapsed() > self.reset_timeout {
                info!("Circuit Breaker: Half-Open. Testing service recovery...");
                *state = CircuitState::HalfOpen;
            } else {
                warn!("Circuit Breaker is OPEN. Request blocked to prevent cascading failure.");
                return Err(AppError::InternalError("Service Unavailable (Circuit Open)".into()));
            }
        }

        drop(state);

        // Execute the action
        match action().await {
            Ok(result) => {
                self.on_success().await;
                Ok(result)
            }
            Err(e) => {
                self.on_failure().await;
                Err(e)
            }
        }
    }

    async fn on_success(&self) {
        let mut state = self.state.lock().await;
        if *state == CircuitState::HalfOpen {
            info!("Circuit Breaker: CLOSED. Service recovered.");
            *state = CircuitState::Closed;
            self.failure_count.store(0, Ordering::SeqCst);
        }
    }

    async fn on_failure(&self) {
        let failures = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;
        if failures >= self.failure_threshold {
            let mut state = self.state.lock().await;
            if *state != CircuitState::Open {
                warn!("Circuit Breaker: OPEN. Failure threshold reached.");
                *state = CircuitState::Open;
                *self.last_failure.lock().await = Some(Instant::now());
            }
        }
    }
}
