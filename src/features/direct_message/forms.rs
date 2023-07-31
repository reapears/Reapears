//! Direct Message form impls

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::types::ModelID;

use super::models::DirectMessage;

/// Message sent from server to user(s) via websocket.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "body")]
pub enum ForwardMessage {
    DirectMessage(DirectMessage),
    MessageIsRead(MessageIsRead),
    UserConnected(ModelID),
    UserDisconnected(ModelID),
    IncomingMessageError(IncomingMessageError),
}

/// Message sent from client to server via websocket.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "body")]
pub enum IncomingMessage {
    NewMessage(NewMessage),
    MessageIsRead(MessageIsRead),
    MessageDelete(MessageDelete),
    UserConnected,
    UserDisconnected,
}

// ===== Variant impls =====

/// New direct message ws request
#[derive(Debug, Clone, Deserialize)]
pub struct NewMessage {
    pub content: String,
    pub receiver_id: String,
}

/// New Message cleaned data
#[derive(Debug, Clone)]
pub struct NewMessageInsertData {
    pub id: ModelID,
    pub sender_id: ModelID,
    pub receiver_id: ModelID,
    pub content: String,
    pub sent_at: OffsetDateTime,
    pub status: NewMessageStatusInsertData,
}

/// New Message metadata
#[derive(Debug, Clone)]
pub struct NewMessageStatusInsertData {
    pub message_id: ModelID,
    pub is_read: bool,
    pub sender_has_deleted: bool,
    pub receiver_has_deleted: bool,
}

impl NewMessage {
    /// Convert `Self` into `NewMessageInsertData`
    #[must_use]
    pub fn insert_data(self, user_id: ModelID) -> NewMessageInsertData {
        let message_id = ModelID::new();
        NewMessageInsertData {
            id: message_id,
            sender_id: user_id,
            receiver_id: ModelID::from_str_unchecked(self.receiver_id),
            content: self.content,
            sent_at: OffsetDateTime::now_utc(),
            status: NewMessageStatusInsertData::new(message_id),
        }
    }
}

impl NewMessageInsertData {
    /// Convert `Self` into `DirectMessage`
    #[must_use]
    pub fn direct_message(&self) -> DirectMessage {
        DirectMessage {
            id: self.id,
            sender_id: self.sender_id,
            receiver_id: self.receiver_id,
            content: self.content.clone(),
            sent_at: self.sent_at,
            is_author: false,
            is_read: false,
        }
    }
}

impl NewMessageStatusInsertData {
    /// Creates a new `MessageStatus` for `NewMessage`.
    #[must_use]
    pub fn new(message_id: ModelID) -> Self {
        Self {
            message_id,
            is_read: true,
            sender_has_deleted: false,
            receiver_has_deleted: false,
        }
    }
}

/// Update messages are `is_read` ws request
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageIsRead {
    pub sender_id: ModelID,
    pub messages: Vec<ModelID>,
}

/// Delete direct message ws request
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDelete {
    pub message_id: ModelID,
}

/// An error sent to the user which
/// encountered while working with an `IncomingMessage`.
#[derive(Debug, Clone, Serialize)]
// #[serde(tag = "code")]
pub enum IncomingMessageError {
    /// `IncomingMessage` could not be deserialized error.
    UnprocessableEntity,
    /// Server error occurs while working with an `IncomingMessage`.
    InternalServerError,
    // NotFound,
    // Forbidden,
    // BadRequest(String),
}
