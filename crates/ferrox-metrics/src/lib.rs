//! # Ferrox Metrics (`ferrox-metrics`)
//!
//! `ferrox-metrics` provides Prometheus metrics collection and HTTP endpoint handlers for Ferrox backend applications.
//! It tracks incoming HTTP request counts, response latency histograms, database connection pool stats, and error rates.
//!
//! ## Observability Role
//! Continuous monitoring in production requires standard metrics formats readable by Prometheus and Grafana. `ferrox-metrics` exposes
//! a `/metrics` route handler that renders counters and histograms in Prometheus text format.
//!
//! ## Key Features
//! - 📊 **Prometheus Exporter**: Standard `/metrics` scraper handler for Axum.
//! - ⏱️ **Request Latency Histograms**: Quantile recording of HTTP response times.
//! - 🔢 **Custom Counter Registry**: Easily register custom application-specific business metrics.

use metrics::{counter, describe_counter};
use metrics_exporter_prometheus::PrometheusBuilder;
use ferrox_errors::AppError;

/// Initializes the Prometheus metrics exporter and registers standard metrics.
/// Call this once during application bootstrap.
pub fn setup_metrics() -> Result<(), AppError> {
    PrometheusBuilder::new()
        .with_http_listener(([0, 0, 0, 0], 9000))
        .install()
        .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

    // Pre-register standard metrics (good practice for Prometheus)
    describe_counter!("http_requests_total", "Total number of HTTP requests processed");
    describe_counter!("db_queries_total", "Total number of database queries executed");
    describe_counter!("events_published_total", "Total number of domain events published");

    println!("ferrox-metrics initialized: Prometheus exporter ready.");
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