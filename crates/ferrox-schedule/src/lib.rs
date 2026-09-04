use ferrox_errors::AppError;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

pub struct Scheduler {
    scheduler: JobScheduler,
}

impl Scheduler {
    pub async fn new() -> Result<Self, AppError> {
        let scheduler = JobScheduler::new().await
            .map_err(|e| AppError::InternalError(format!("Scheduler init failed: {}", e)))?;
        Ok(Self { scheduler })
    }

    /// Adds a Cron Job to the scheduler.
    /// Expression format: "sec min hour day_of_month month day_of_week year"
    /// Example: "0 * * * * * *" (Every minute)
    pub async fn add_job<F>(&mut self, expression: &str, task: F) -> Result<(), AppError>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let job = Job::new(expression, move |_uuid, _l| {
            task();
        }).map_err(|e| AppError::InternalError(format!("Job creation failed: {}", e)))?;

        self.scheduler.add(job).await
            .map_err(|e| AppError::InternalError(format!("Job addition failed: {}", e)))?;
            
        info!("Cron job registered: {}", expression);
        Ok(())
    }

    pub async fn start(&self) -> Result<(), AppError> {
        self.scheduler.start().await
            .map_err(|e| AppError::InternalError(format!("Scheduler start failed: {}", e)))?;
        info!("Ferrox Scheduler started in background.");
        Ok(())
    }
}
