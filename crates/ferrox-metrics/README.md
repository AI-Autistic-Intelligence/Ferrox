# Ferrox Metrics (`ferrox-metrics`)

`ferrox-metrics` provides Prometheus metrics collection and HTTP endpoint handlers for Ferrox backend applications.
It tracks incoming HTTP request counts, response latency histograms, database connection pool stats, and error rates.

## Observability Role
Continuous monitoring in production requires standard metrics formats readable by Prometheus and Grafana. `ferrox-metrics` exposes
a `/metrics` route handler that renders counters and histograms in Prometheus text format.

## Key Features
- 📊 **Prometheus Exporter**: Standard `/metrics` scraper handler for Axum.
- ⏱️ **Request Latency Histograms**: Quantile recording of HTTP response times.
- 🔢 **Custom Counter Registry**: Easily register custom application-specific business metrics.
