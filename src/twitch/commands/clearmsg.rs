use super::*;

/// When a single message has been removed from a channel.
///
/// This is triggered via /delete <target-msg-id> on IRC.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClearMsg {
    pub(super) tags: Tags,
    pub(super) channel: Channel,
    pub(super) message: Option<String>,
}

impl ClearMsg {
    /// IRC tags
    pub fn tags(&self) -> &Tags {
        &self.tags
    }
    /// The channel this event happened on
    pub fn channel(&self) -> &Channel {
        &self.channel
    }
    /// The message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_ref().map(String::as_str)
    }
}

impl ClearMsg {
    /// Name of the user who sent the message.
    pub fn login(&self) -> Option<&str> {
        self.get("login")
    }
    /// UUID of the message.
    pub fn target_msg_id(&self) -> Option<&str> {
        self.get("target-msg-id")
    }
}

impl Tag for ClearMsg {
    fn get(&self, key: &str) -> Option<&str> {
        self.tags.get(key).map(AsRef::as_ref)
    }
}
