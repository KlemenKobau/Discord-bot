use crate::config::Config;
use anyhow::Result;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Initialize the logging system with optional Loki integration
pub fn init(config: &Config) -> Result<()> {
    let fmt_layer = tracing_subscriber::fmt::layer();

    // Check if Loki is configured
    if let Some(loki_url) = &config.loki_url {
        init_with_loki(fmt_layer, loki_url, config)
    } else {
        init_console_only(fmt_layer)
    }
}

/// Initialize logging with Loki integration
fn init_with_loki(
    fmt_layer: tracing_subscriber::fmt::Layer<tracing_subscriber::Registry>,
    loki_url: &str,
    config: &Config,
) -> Result<()> {
    info!("Loki URL configured: {}", loki_url);

    // Parse the Loki URL
    let mut url = url::Url::parse(loki_url)?;

    // Add authentication if provided (for Grafana Cloud)
    if let (Some(username), Some(api_key)) = (&config.loki_username, &config.loki_api_key) {
        url.set_username(username)
            .map_err(|_| anyhow::anyhow!("Failed to set Loki username"))?;
        url.set_password(Some(api_key))
            .map_err(|_| anyhow::anyhow!("Failed to set Loki API key"))?;
        info!("Loki authentication configured for user: {}", username);
    }

    // Build the Loki layer with labels
    let (loki_layer, task) = tracing_loki::builder()
        .label("service", "discord-bot")?
        .label("environment", &config.environment)?
        .build_url(url)?;

    // Spawn the Loki background task
    tokio::spawn(task);

    // Initialize the subscriber with both console and Loki layers
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(loki_layer)
        .init();

    info!("Logging initialized with Loki integration");
    Ok(())
}

/// Initialize console-only logging
fn init_console_only(
    fmt_layer: tracing_subscriber::fmt::Layer<tracing_subscriber::Registry>,
) -> Result<()> {
    tracing_subscriber::registry().with(fmt_layer).init();

    info!("Logging initialized (Loki disabled - set LOKI_URL to enable)");
    Ok(())
}
