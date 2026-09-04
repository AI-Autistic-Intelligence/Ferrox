use opentelemetry::KeyValue;
use opentelemetry_sdk::trace::{self, Sampler};
use opentelemetry_sdk::Resource;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use yalc_errors::AppError;

pub struct LoggerConfig {
    pub service_name: String,
    pub otlp_endpoint: Option<String>,
    pub sentry_dsn: Option<String>,
}

pub fn setup_logger(config: LoggerConfig) -> Result<Option<sentry::ClientInitGuard>, AppError> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    
    // Default JSON formatting for standard output
    let formatting_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_file(true)
        .with_line_number(true)
        .with_target(false); // We often don't need the module target in JSON if we have file/line

    let subscriber = Registry::default().with(env_filter).with(formatting_layer);

    // If Sentry DSN is provided, setup Sentry
    let mut sentry_guard = None;
    if let Some(dsn) = config.sentry_dsn {
        sentry_guard = Some(sentry::init((
            dsn,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                traces_sample_rate: 1.0,
                ..Default::default()
            },
        )));
        
        let sentry_layer = sentry_tracing::layer();
        
        if let Some(endpoint) = config.otlp_endpoint {
            let tracer = opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(
                    opentelemetry_otlp::new_exporter()
                        .tonic()
                        .with_endpoint(endpoint),
                )
                .with_trace_config(
                    trace::config()
                        .with_sampler(Sampler::AlwaysOn)
                        .with_resource(Resource::new(vec![KeyValue::new("service.name", config.service_name)])),
                )
                .install_batch(opentelemetry_sdk::runtime::Tokio)
                .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

            let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
            
            subscriber.with(telemetry_layer).with(sentry_layer).init();
        } else {
            subscriber.with(sentry_layer).init();
        }
    } else {
        if let Some(endpoint) = config.otlp_endpoint {
            let tracer = opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(
                    opentelemetry_otlp::new_exporter()
                        .tonic()
                        .with_endpoint(endpoint),
                )
                .with_trace_config(
                    trace::config()
                        .with_sampler(Sampler::AlwaysOn)
                        .with_resource(Resource::new(vec![KeyValue::new("service.name", config.service_name)])),
                )
                .install_batch(opentelemetry_sdk::runtime::Tokio)
                .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

            let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
            
            subscriber.with(telemetry_layer).init();
        } else {
            subscriber.init();
        }
    }

    tracing::info!("yalc-logger initialized: Structured JSON logging enabled.");
    Ok(sentry_guard)
}
