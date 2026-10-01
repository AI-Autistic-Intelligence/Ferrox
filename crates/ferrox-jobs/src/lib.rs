//! # Ferrox Jobs (`ferrox-jobs`)
//!
//! `ferrox-jobs` provides async background job processing powered by Redis and Apalis, enabling persistent task queues with retry logic.

use apalis::{prelude::*, redis::RedisStorage};
use serde::{Deserialize, Serialize};
use ferrox_errors::AppError;

/// A simple Job representation
#[derive(Debug, Deserialize, Serialize)]
pub struct BackgroundJob {
    pub task_name: String,
    pub payload: String,
}

impl Job for BackgroundJob {
    const NAME: &'static str = "ferrox::BackgroundJob";
}

pub async fn start_worker(redis_url: &str) -> Result<(), AppError> {
    let storage = RedisStorage::new(
        apalis::redis::Config::default().set_url(redis_url)
    )
    .await
    .map_err(|e| AppError::Internal(format!("Redis Job Storage Error: {}", e)))?;

    tracing::info!("Starting Apalis Worker on {}", redis_url);
    
    let worker = WorkerBuilder::new("ferrox-default-worker")
        .with_storage(storage)
        .build_fn(process_job);

    apalis::prelude::Monitor::new()
        .register(worker)
        .run()
        .await
        .map_err(|e| AppError::Internal(format!("Worker crashed: {}", e)))?;

    Ok(())
}

/// The actual job processing logic
async fn process_job(job: BackgroundJob, _ctx: apalis::prelude::Context<()>) -> Result<(), apalis::prelude::Error> {
    tracing::info!("Processing Job: {} with payload: {}", job.task_name, job.payload);
    // Add real execution logic here
    Ok(())
}