use crate::config::Config;
use anyhow::Result;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::KeyValue;
use opentelemetry_otlp::{WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_sdk::Resource;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

/// Initialize the logging system with optional OpenTelemetry integration
pub fn init(config: &Config) -> Result<()> {
    // Create environment filter that respects RUST_LOG
    // Defaults to "info" if RUST_LOG is not set
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Check if OpenTelemetry is configured
    if config.has_otlp() {
        init_with_otlp(env_filter, config)
    } else {
        init_console_only(env_filter)
    }
}

/// Initialize logging with OpenTelemetry OTLP integration
fn init_with_otlp(env_filter: EnvFilter, config: &Config) -> Result<()> {
    let otlp_endpoint = config
        .otlp_endpoint
        .as_ref()
        .expect("OTLP endpoint should be present");

    info!("OpenTelemetry OTLP endpoint configured: {}", otlp_endpoint);

    // Build the OTLP exporter with optional headers and timeout
    let mut exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_endpoint)
        .with_timeout(std::time::Duration::from_secs(30)); // Increase timeout to 30 seconds

    // Add custom headers if configured (e.g., for authentication)
    if let Some(headers) = config.parse_otlp_headers() {
        let mut metadata = tonic::metadata::MetadataMap::new();
        for (key, value) in &headers {
            info!("Adding OTLP header: {} = {}", key, if key == "Authorization" { "***" } else { value });
            if let Ok(meta_key) = tonic::metadata::MetadataKey::from_bytes(key.as_bytes()) {
                if let Ok(meta_value) = value.parse() {
                    metadata.insert(meta_key, meta_value);
                }
            }
        }
        exporter = exporter.with_metadata(metadata);
    } else {
        info!("No OTLP headers configured");
    }

    // Create resource with service information
    let resource = Resource::new(vec![
        KeyValue::new("service.name", "discord-bot"),
        KeyValue::new("service.environment", config.environment.clone()),
    ]);

    // Build the tracer provider
    let tracer_provider = TracerProvider::builder()
        .with_batch_exporter(exporter.build()?, opentelemetry_sdk::runtime::Tokio)
        .with_resource(resource)
        .build();

    // Create the OpenTelemetry tracing layer
    let tracer = tracer_provider.tracer("kendo-bot");
    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // Initialize the subscriber with environment filter, console, and OpenTelemetry layers
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .with(otel_layer)
        .init();

    info!("Logging initialized with OpenTelemetry OTLP integration");
    Ok(())
}

/// Initialize console-only logging
fn init_console_only(env_filter: EnvFilter) -> Result<()> {
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Logging initialized (OpenTelemetry disabled - set OTLP_ENDPOINT to enable)");
    Ok(())
}
