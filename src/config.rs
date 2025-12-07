use anyhow::{Context, Result};
use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// Discord bot token
    pub discord_token: String,

    /// OpenTelemetry OTLP endpoint URL (optional)
    pub otlp_endpoint: Option<String>,

    /// OpenTelemetry headers for authentication (optional, format: "key1=value1,key2=value2")
    pub otlp_headers: Option<String>,

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

        let otlp_endpoint = env::var("OTLP_ENDPOINT").ok();
        let otlp_headers = env::var("OTLP_HEADERS").ok();
        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "production".to_string());

        Ok(Self {
            discord_token,
            otlp_endpoint,
            otlp_headers,
            environment,
        })
    }

    /// Check if OpenTelemetry is configured
    pub fn has_otlp(&self) -> bool {
        self.otlp_endpoint.is_some()
    }

    /// Parse OTLP headers from the configuration string
    /// Expected format: "key1=value1,key2=value2"
    pub fn parse_otlp_headers(&self) -> Option<Vec<(String, String)>> {
        self.otlp_headers.as_ref().map(|headers| {
            headers
                .split(',')
                .filter_map(|pair| {
                    let mut parts = pair.split('=');
                    match (parts.next(), parts.next()) {
                        (Some(key), Some(value)) => {
                            Some((key.trim().to_string(), value.trim().to_string()))
                        }
                        _ => None,
                    }
                })
                .collect()
        })
    }
}
