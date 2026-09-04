use dashmap::DashMap;
use ferrox_errors::AppError;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{info, debug};

/// Singleflight prevents the "Cache Stampede" effect (Dogpile effect).
/// If multiple requests for the same key arrive simultaneously, only the first one
/// executes the future. The others wait and receive the result of the first one.
#[derive(Clone)]
pub struct Singleflight<T> {
    // Maps a cache key to a broadcast channel that will receive the result
    in_flight: Arc<DashMap<String, broadcast::Sender<Result<T, String>>>>,
}

impl<T: Clone + Send + Sync + 'static> Singleflight<T> {
    pub fn new() -> Self {
        Self {
            in_flight: Arc::new(DashMap::new()),
        }
    }

    /// Executes the provided async closure `fut` for the given `key`, or waits for an existing
    /// execution to finish and returns its result.
    pub async fn execute<F, Fut>(&self, key: &str, fut: F) -> Result<T, AppError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, AppError>> + Send + 'static,
    {
        // 1. Check if the key is already in-flight
        let rx = {
            if let Some(tx) = self.in_flight.get(key) {
                // Another thread is already computing this. We just subscribe to the result.
                Some(tx.subscribe())
            } else {
                // We are the FIRST thread. Create a broadcast channel.
                let (tx, _rx) = broadcast::channel(1);
                self.in_flight.insert(key.to_string(), tx);
                None
            }
        };

        // 2. If we got a receiver, await the result broadcasted by the first thread
        if let Some(mut receiver) = rx {
            debug!("Singleflight: Suspending execution. Waiting for in-flight result for key: {}", key);
            return match receiver.recv().await {
                Ok(Ok(val)) => Ok(val),
                Ok(Err(e)) => Err(AppError::InternalError(e)),
                Err(_) => Err(AppError::InternalError("Singleflight sender dropped".into())),
            };
        }

        // 3. We are the first thread. We MUST execute the future.
        info!("Singleflight: Primary execution started for key: {}", key);
        let result = fut().await;

        // 4. Broadcast the result to all waiters and remove the key
        if let Some((_, tx)) = self.in_flight.remove(key) {
            let broadcast_payload = match &result {
                Ok(val) => Ok(val.clone()),
                Err(e) => Err(format!("{:?}", e)),
            };
            // Ignore error if there are no receivers (it means 0 stampede occurred)
            let _ = tx.send(broadcast_payload);
        }

        result
    }
}
