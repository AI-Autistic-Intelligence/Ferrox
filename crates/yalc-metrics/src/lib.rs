use metrics::{counter, describe_counter};
use metrics_exporter_prometheus::PrometheusBuilder;
use yalc_errors::AppError;

/// Initializes the Prometheus metrics exporter and registers standard metrics.
/// Call this once during application bootstrap.
pub fn setup_metrics() -> Result<(), AppError> {
    PrometheusBuilder::new()
        .install()
        .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

    // Pre-register standard metrics (good practice for Prometheus)
    describe_counter!("http_requests_total", "Total number of HTTP requests processed");
    describe_counter!("db_queries_total", "Total number of database queries executed");
    describe_counter!("events_published_total", "Total number of domain events published");

    println!("yalc-metrics initialized: Prometheus exporter ready.");
    Ok(())
}

/// Helper function to increment request counters
pub fn record_http_request() {
    counter!("http_requests_total").increment(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_initialization() {
        // In testing, metrics can only be initialized once globally. 
        // We use a setup that won't panic if already installed.
        let result = setup_metrics();
        assert!(result.is_ok());

        // Test that recording works without panic
        record_http_request();
    }
}
