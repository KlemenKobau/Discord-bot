use anyhow::Result;
use kobi_kendo_discord_bot::{roles::RoleReaction, BotHandler, Config};
use serenity::prelude::*;
use tracing::{error, info};

// Bot configuration constants
const MONITORED_MESSAGE: u64 = 1438242531782561844;
const ANIME_ROLE_ID: u64 = 942341466540355584;
const ANIME_ROLE_EMOJI: &str = "🎎";

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration from environment
    let config = Config::from_env()?;

    // Initialize logging (console + optional Loki)
    kobi_kendo_discord_bot::logging::init(&config)?;

    info!("Starting Discord bot...");

    // Configure role reactions
    let role_reactions = vec![RoleReaction::new(
        MONITORED_MESSAGE,
        ANIME_ROLE_ID,
        ANIME_ROLE_EMOJI,
    )];

    // Set up gateway intents
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::AUTO_MODERATION_CONFIGURATION;

    // Build the Discord client
    let mut client = Client::builder(&config.discord_token, intents)
        .event_handler(BotHandler::new(role_reactions))
        .await?;

    // Start the bot with graceful shutdown support
    tokio::select! {
        result = client.start() => {
            if let Err(err) = result {
                error!("Discord client error: {:?}", err);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl-C, shutting down gracefully...");
        }
    }

    Ok(())
}
