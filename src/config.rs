use anyhow::{Context, Result};
use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// Discord bot token
    pub discord_token: String,

    /// Loki endpoint URL (optional)
    pub loki_url: Option<String>,

    /// Loki username for Grafana Cloud (optional)
    pub loki_username: Option<String>,

    /// Loki API key for Grafana Cloud (optional)
    pub loki_api_key: Option<String>,

    /// Environment label for logs (defaults to "production")
    pub environment: String,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        // Load .env file if it exists (for local development)
        dotenvy::dotenv().ok();

        let discord_token = env::var("DISCORD_TOKEN")
            .context("DISCORD_TOKEN environment variable is required")?;

        let loki_url = env::var("LOKI_URL").ok();
        let loki_username = env::var("LOKI_USERNAME").ok();
        let loki_api_key = env::var("LOKI_API_KEY").ok();
        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "production".to_string());

        Ok(Self {
            discord_token,
            loki_url,
            loki_username,
            loki_api_key,
            environment,
        })
    }

    /// Check if Loki logging is configured
    pub fn has_loki(&self) -> bool {
        self.loki_url.is_some()
    }

    /// Check if Grafana Cloud authentication is configured
    pub fn has_loki_auth(&self) -> bool {
        self.loki_username.is_some() && self.loki_api_key.is_some()
    }
}
