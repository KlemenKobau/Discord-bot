use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub discord_token: String,
    pub otlp_endpoint: Option<String>,
    pub otlp_headers: Option<String>,
    pub environment: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
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

    pub fn has_otlp(&self) -> bool {
        self.otlp_endpoint.is_some()
    }

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
