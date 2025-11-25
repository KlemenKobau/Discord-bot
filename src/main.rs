use anyhow::Result;
use serenity::all::Reaction;
use serenity::all::{Member, MessageId};
use serenity::async_trait;
use serenity::http::Http;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
use tracing::{error, info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const MONITORED_MESSAGE: u64 = 1438242531782561844;
const ANIME_ROLE_ID: u64 = 942341466540355584;
const ANIME_ROLE_EMOJI: &str = "🎎";

struct Bot;

async fn get_member(http: &Http, reaction: &Reaction) -> Option<Member> {
    let user_id = reaction.user_id;
    let guild_id = reaction.guild_id;

    if user_id.is_none() {
        warn!("User id is none in reaction: {:?}", reaction);
        return None;
    }

    if guild_id.is_none() {
        warn!("Guild id none in reaction: {:?}", reaction);
        return None;
    }

    let member = http.get_member(guild_id.unwrap(), user_id.unwrap()).await;
    match member {
        Ok(member) => Some(member),
        Err(err) => {
            warn!("Error getting member: {}", err);
            None
        }
    }
}

fn is_message_relevant_for_bot(reaction: &Reaction) -> bool {
    let bot_monitored_message = MessageId::new(MONITORED_MESSAGE);

    if !reaction.message_id.eq(&bot_monitored_message) {
        return false;
    }

    if !reaction.emoji.unicode_eq(ANIME_ROLE_EMOJI) {
        return false;
    }

    true
}

#[async_trait]
impl EventHandler for Bot {
    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        if !is_message_relevant_for_bot(&add_reaction) {
            return;
        }

        let http = ctx.http();
        let member = get_member(http, &add_reaction).await;

        if member.is_none() {
            warn!("Member is none: {:?}", add_reaction);
            return;
        }

        let member = member.unwrap();

        info!("Assigning role to member: {}", member.display_name());

        let role_add_res = member.add_role(http, ANIME_ROLE_ID).await;

        if let Err(err) = role_add_res {
            error!("Error adding role: {}", err);
        }
    }

    async fn reaction_remove(&self, ctx: Context, removed_reaction: Reaction) {
        if !is_message_relevant_for_bot(&removed_reaction) {
            return;
        }

        let http = ctx.http();
        let member = get_member(http, &removed_reaction).await;

        if member.is_none() {
            warn!("Member is none: {:?}", removed_reaction);
            return;
        }

        let member = member.unwrap();

        info!("Removing role from member: {}", member.display_name());

        let role_add_res = member.remove_role(http, ANIME_ROLE_ID).await;

        if let Err(err) = role_add_res {
            error!("Error removing role: {}", err);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file (if it exists)
    dotenvy::dotenv().ok();

    // Initialize logging with optional Loki integration
    let fmt_layer = tracing_subscriber::fmt::layer();

    // Check if Loki URL is configured
    let loki_url = env::var("LOKI_URL").ok();

    if let Some(url) = loki_url {
        info!("Loki URL configured: {}", url);

        // Parse Loki URL
        let url = url::Url::parse(&url).expect("Invalid LOKI_URL format");

        // Build Loki layer with labels
        let (loki_layer, task) = tracing_loki::builder()
            .label("service", "discord-bot")
            .expect("Failed to set service label")
            .label("environment", env::var("ENVIRONMENT").unwrap_or_else(|_| "production".to_string()))
            .expect("Failed to set environment label")
            .build_url(url)
            .expect("Failed to build Loki layer");

        // Spawn the Loki task to send logs in the background
        tokio::spawn(task);

        // Initialize subscriber with both fmt and Loki layers
        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(loki_layer)
            .init();

        info!("Logging initialized with Loki integration");
    } else {
        // Initialize with just fmt layer (console logging)
        tracing_subscriber::registry()
            .with(fmt_layer)
            .init();

        info!("Logging initialized (Loki disabled - set LOKI_URL to enable)");
    }

    // Get the discord token from environment
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN environment variable");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::AUTO_MODERATION_CONFIGURATION;

    let mut client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Error creating client");

    // Start listening for events by starting a single shard
    info!("Starting bot...");

    // Spawn the client in a separate task so we can handle shutdown signals
    tokio::select! {
        result = client.start() => {
            if let Err(why) = result {
                error!("Client error: {:?}", why);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl-C, shutting down gracefully...");
        }
    }

    Ok(())
}
