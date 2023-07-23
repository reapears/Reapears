//! Direct Message models impls

use std::sync::Arc;

use serde::Serialize;
use time::OffsetDateTime;
use tokio::sync::broadcast;

use crate::{
    error::{ServerError, ServerResult},
    types::ModelID,
};

use super::forms::IncomingChat;

/// Listen for incoming message and channels them
#[derive(Debug, Clone)]
pub struct ChatBroadcast(Arc<broadcast::Sender<IncomingChat>>);

impl Default for ChatBroadcast {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatBroadcast {
    /// Creates a new `ChatBroadcast` router
    #[must_use]
    pub fn new() -> Self {
        let (sender, _recv) = broadcast::channel(1024);
        Self(Arc::new(sender))
    }

    /// Sends an `IncomingChat` to the queue
    pub fn send(&self, msg: IncomingChat) {
        let _ = self.0.send(msg);
    }

    /// Returns new a receiver for incoming messages
    #[must_use]
    pub fn message_listener(&self) -> ChatMessages {
        ChatMessages(self.0.subscribe())
    }
}

/// Chat --
#[derive(Debug)]
pub struct ChatMessages(broadcast::Receiver<IncomingChat>);

impl ChatMessages {
    pub async fn recv(&mut self) -> ServerResult<IncomingChat> {
        self.0
            .recv()
            .await
            .map_err(|err| ServerError::new(err.to_string()))
    }
}

// ===== Direct Message impls =====

/// Direct Message sent between two users.
#[derive(Debug, Clone, Serialize)]
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

/// Direct messages sent between two users
#[derive(Debug, Clone, Serialize)]
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

/// A list of Conversations the user had
/// Ordered by the most recent first
#[derive(Debug, Clone)]
pub struct Conversations(Vec<Conversation>);

impl Conversations {
    /// Creates a new `Conversations`
    #[must_use]
    pub fn from_row(mut conversations: Vec<Conversation>) -> Self {
        conversations.sort_by_key(Conversation::ordering_key);
        Self(conversations)
    }
}
