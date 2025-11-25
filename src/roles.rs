use serenity::all::{Member, MessageId, Reaction, RoleId};
use serenity::http::Http;
use tracing::warn;

/// Configuration for a role-based reaction
#[derive(Debug, Clone)]
pub struct RoleReaction {
    /// The message ID to monitor for reactions
    pub message_id: MessageId,
    /// The role ID to assign/remove
    pub role_id: RoleId,
    /// The emoji that triggers the role assignment
    pub emoji: String,
}

impl RoleReaction {
    /// Create a new role reaction configuration
    pub fn new(message_id: u64, role_id: u64, emoji: impl Into<String>) -> Self {
        Self {
            message_id: MessageId::new(message_id),
            role_id: RoleId::new(role_id),
            emoji: emoji.into(),
        }
    }

    /// Check if a reaction matches this role reaction configuration
    pub fn matches(&self, reaction: &Reaction) -> bool {
        reaction.message_id == self.message_id && reaction.emoji.unicode_eq(&self.emoji)
    }
}

/// Get a member from a reaction
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

/// Add a role to a member
pub async fn add_role(http: &Http, member: &Member, role_id: RoleId) -> Result<(), serenity::Error> {
    member.add_role(http, role_id).await
}

/// Remove a role from a member
pub async fn remove_role(
    http: &Http,
    member: &Member,
    role_id: RoleId,
) -> Result<(), serenity::Error> {
    member.remove_role(http, role_id).await
}
