use serenity::all::{Member, MessageId, Reaction, RoleId};
use serenity::http::Http;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct RoleReaction {
    pub message_id: MessageId,
    pub role_id: RoleId,
    pub emoji: String,
}

impl RoleReaction {
    pub fn new(message_id: u64, role_id: u64, emoji: impl Into<String>) -> Self {
        Self {
            message_id: MessageId::new(message_id),
            role_id: RoleId::new(role_id),
            emoji: emoji.into(),
        }
    }

    pub fn matches(&self, reaction: &Reaction) -> bool {
        reaction.message_id == self.message_id && reaction.emoji.unicode_eq(&self.emoji)
    }
}

pub async fn get_member(http: &Http, reaction: &Reaction) -> Option<Member> {
    let user_id = reaction.user_id?;
    let guild_id = reaction.guild_id?;

    match http.get_member(guild_id, user_id).await {
        Ok(member) => Some(member),
        Err(err) => {
            warn!(
                "Failed to get member for user_id={} guild_id={}: {}",
                user_id, guild_id, err
            );
            None
        }
    }
}

pub async fn add_role(http: &Http, member: &Member, role_id: RoleId) -> Result<(), serenity::Error> {
    member.add_role(http, role_id).await
}

pub async fn remove_role(
    http: &Http,
    member: &Member,
    role_id: RoleId,
) -> Result<(), serenity::Error> {
    member.remove_role(http, role_id).await
}
