use crate::roles::{self, RoleReaction};
use serenity::all::Reaction;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{error, info, warn};

pub struct BotHandler {
    role_reactions: Vec<RoleReaction>,
}

impl BotHandler {
    pub fn new(role_reactions: Vec<RoleReaction>) -> Self {
        Self { role_reactions }
    }

    fn is_relevant_reaction(&self, reaction: &Reaction) -> Option<&RoleReaction> {
        self.role_reactions
            .iter()
            .find(|config| config.matches(reaction))
    }
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let Some(role_config) = self.is_relevant_reaction(&reaction) else {
            return;
        };

        let http = ctx.http();

        let Some(member) = roles::get_member(http, &reaction).await else {
            warn!("Could not retrieve member for reaction: {:?}", reaction);
            return;
        };

        info!(
            "Assigning role {} to member: {}",
            role_config.role_id,
            member.display_name()
        );

        if let Err(err) = roles::add_role(http, &member, role_config.role_id).await {
            error!(
                "Failed to add role {} to member {}: {}",
                role_config.role_id,
                member.display_name(),
                err
            );
        }
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        let Some(role_config) = self.is_relevant_reaction(&reaction) else {
            return;
        };

        let http = ctx.http();

        let Some(member) = roles::get_member(http, &reaction).await else {
            warn!("Could not retrieve member for reaction: {:?}", reaction);
            return;
        };

        info!(
            "Removing role {} from member: {}",
            role_config.role_id,
            member.display_name()
        );

        if let Err(err) = roles::remove_role(http, &member, role_config.role_id).await {
            error!(
                "Failed to remove role {} from member {}: {}",
                role_config.role_id,
                member.display_name(),
                err
            );
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}
