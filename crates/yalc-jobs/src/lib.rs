use apalis::{prelude::*, redis::RedisStorage};
use serde::{Deserialize, Serialize};
use yalc_errors::AppError;

/// A simple Job representation
#[derive(Debug, Deserialize, Serialize)]
pub struct BackgroundJob {
    pub task_name: String,
    pub payload: String,
}

impl Job for BackgroundJob {
    const NAME: &'static str = "yalc::BackgroundJob";
}

/// Helper to configure and run the Apalis worker
pub async fn start_worker(redis_url: &str) -> Result<(), AppError> {
    let storage = RedisStorage::new(redis_url.to_string())
        .await
        .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

    // Create a background task that processes jobs
    tokio::spawn(async move {
        tracing::info!("Starting Background Job Worker...");
        Monitor::new()
            .register_with_count(2, move |c| {
                WorkerBuilder::new(format!("yalc-worker-{}", c))
                    .with_storage(storage.clone())
                    .build_fn(process_job)
            })
            .run()
            .await
            .unwrap_or_else(|e| tracing::error!("Job worker failed: {:?}", e));
    });

    Ok(())
}

/// The actual job processing logic
async fn process_job(job: BackgroundJob, _ctx: JobContext) -> Result<(), apalis::prelude::Error> {
    tracing::info!("Processing Job: {} with payload: {}", job.task_name, job.payload);
    // Add real execution logic here
    Ok(())
}
