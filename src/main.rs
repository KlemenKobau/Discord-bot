use anyhow::anyhow;
use serenity::all::Reaction;
use serenity::all::{Member, MessageId};
use serenity::async_trait;
use serenity::http::Http;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info, warn};

const MONITORED_MESSAGE: u64 = 1183140575361368064;
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

        let role_add_res = member.unwrap().add_role(http, ANIME_ROLE_ID).await;

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

        let role_add_res = member.unwrap().remove_role(http, ANIME_ROLE_ID).await;

        if let Err(err) = role_add_res {
            error!("Error removing role: {}", err);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::AUTO_MODERATION_CONFIGURATION;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
