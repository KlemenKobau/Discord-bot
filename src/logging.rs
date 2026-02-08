use crate::config::Config;
use anyhow::{Context, Result};
use opentelemetry::KeyValue;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::logs::LoggerProvider;
use opentelemetry_sdk::Resource;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub fn init(config: &Config) -> Result<Option<LoggerProvider>> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    if config.has_otlp() {
        init_with_otlp(env_filter, config).map(Some)
    } else {
        init_console_only(env_filter)?;
        Ok(None)
    }
}

fn init_with_otlp(env_filter: EnvFilter, config: &Config) -> Result<LoggerProvider> {
    let otlp_endpoint = config
        .otlp_endpoint
        .as_ref()
        .context("OTLP endpoint is required but not configured")?;

    eprintln!("OpenTelemetry OTLP endpoint configured: {}", otlp_endpoint);

    let resource = Resource::new(vec![
        KeyValue::new("service.name", "kobi-kendo-discord-bot"),
        KeyValue::new("service.environment", config.environment.clone()),
    ]);

    let logs_endpoint = if otlp_endpoint.contains("posthog.com") {
        if otlp_endpoint.contains("eu.posthog.com") {
            "https://eu.i.posthog.com/i/v1/logs"
        } else if otlp_endpoint.contains("us.posthog.com") {
            "https://us.i.posthog.com/i/v1/logs"
        } else {
            otlp_endpoint.as_str()
        }
    } else {
        otlp_endpoint.as_str()
    };

    eprintln!("PostHog logs endpoint: {}", logs_endpoint);

    let mut log_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_http()
        .with_endpoint(logs_endpoint);

    if let Some(headers) = config.parse_otlp_headers() {
        let mut http_headers = std::collections::HashMap::new();
        for (key, value) in headers {
            eprintln!("Adding OTLP log header: {} = {}", key, if key == "Authorization" { "***" } else { &value });
            http_headers.insert(key, value);
        }
        log_exporter = log_exporter.with_headers(http_headers);
    }

    let logger_provider = LoggerProvider::builder()
        .with_resource(resource)
        .with_batch_exporter(log_exporter.build()?, opentelemetry_sdk::runtime::Tokio)
        .build();

    let otel_log_layer = OpenTelemetryTracingBridge::new(&logger_provider);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .with(otel_log_layer)
        .init();

    eprintln!("✅ Logging initialized with OpenTelemetry OTLP integration");
    eprintln!("   - Logs: {} (HTTP)", logs_endpoint);
    eprintln!("   ℹ️  Note: PostHog currently only supports logs, not traces");

    Ok(logger_provider)
}

fn init_console_only(env_filter: EnvFilter) -> Result<()> {
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Logging initialized (OpenTelemetry disabled - set OTLP_ENDPOINT to enable)");
    Ok(())
}
