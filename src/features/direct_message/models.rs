//! Direct Message models impls

use serde::Serialize;
use time::OffsetDateTime;

use crate::types::ModelID;

/// Direct Message sent between two users.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectMessage {
    pub id: ModelID,
    pub sender_id: ModelID,
    pub receiver_id: ModelID,
    pub content: String,
    pub sent_at: OffsetDateTime,
    pub is_author: bool,
    pub is_read: bool,
}

impl DirectMessage {
    /// Creates a new `DirectMessage` from the database row
    #[must_use]
    pub fn from_row(
        id: ModelID,
        sender_id: ModelID,
        receiver_id: ModelID,
        content: String,
        sent_at: OffsetDateTime,
        is_author: bool,
        is_read: bool,
    ) -> Self {
        Self {
            id,
            sender_id,
            receiver_id,
            content,
            sent_at,
            is_author,
            is_read,
        }
    }
}

// ===== Conversation impls =====

/// A list of Conversations the user had
/// Ordered by the most recent first
#[derive(Debug, Clone, Serialize)]
pub struct Conversations(Vec<Conversation>);

impl Conversations {
    /// Creates a new `Conversations`
    #[must_use]
    pub fn from_row(mut conversations: Vec<Conversation>) -> Self {
        conversations.sort_by_key(Conversation::ordering_key);
        Self(conversations)
    }
}

/// Direct messages sent between two users
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
    pub user_id: ModelID,
    pub participant_id: ModelID,
    pub messages: Vec<DirectMessage>,
}

impl Conversation {
    /// Creates a new `Conversation` from the database row.
    #[must_use]
    pub fn from_row(
        user_id: ModelID,
        participant_id: ModelID,
        mut messages: Vec<DirectMessage>,
    ) -> Self {
        messages.sort_by_key(|msg| msg.sent_at); // latest messages at the end
        Self {
            user_id,
            participant_id,
            messages,
        }
    }

    /// Returns the ordering key.
    /// The conversation is ordered by the most
    /// recent message sent which is last message.
    #[must_use]
    pub fn ordering_key(&self) -> OffsetDateTime {
        self.messages
            .last()
            .as_ref()
            .map_or(OffsetDateTime::UNIX_EPOCH, |msg| msg.sent_at)
    }
}
