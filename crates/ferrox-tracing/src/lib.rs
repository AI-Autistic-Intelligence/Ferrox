use opentelemetry::global;
use opentelemetry_sdk::trace::Tracer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

/// Initializes OpenTelemetry and Tracing for Distributed Systems
pub fn init_tracer(service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize OTLP pipeline
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    // 2. Setup tracing subscriber with OTLP layer and console output
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    
    // Add environment filter (e.g. RUST_LOG=info)
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info".into());

    Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    tracing::info!("OpenTelemetry tracing initialized for service: {}", service_name);

    Ok(())
}

pub fn shutdown_tracer() {
    global::shutdown_tracer_provider();
}
