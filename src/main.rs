use anyhow::Result;
use kobi_kendo_discord_bot::{roles::RoleReaction, BotHandler, Config};
use serenity::prelude::*;
use tracing::{error, info};

const MONITORED_MESSAGE: u64 = 1438242531782561844;
const ANIME_ROLE_ID: u64 = 942341466540355584;
const ANIME_ROLE_EMOJI: &str = "🎎";

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env()?;
    let _logger_provider = kobi_kendo_discord_bot::logging::init(&config)?;

    info!("Starting Discord bot...");

    let role_reactions = vec![RoleReaction::new(
        MONITORED_MESSAGE,
        ANIME_ROLE_ID,
        ANIME_ROLE_EMOJI,
    )];

    let intents = GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut client = Client::builder(&config.discord_token, intents)
        .event_handler(BotHandler::new(role_reactions))
        .await?;

    tokio::select! {
        result = client.start() => {
            if let Err(err) = result {
                error!("Discord client error: {:?}", err);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl-C, shutting down gracefully");
        }
    }

    info!("Bot shutdown complete");

    if let Some(provider) = _logger_provider {
        if let Err(e) = provider.shutdown() {
            eprintln!("Error shutting down log provider: {}", e);
        }
    }

    Ok(())
}
