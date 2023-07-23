//! Direct Message form impls

use crate::types::ModelID;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::models::DirectMessage;

/*
NewMessage
MessageIsReadUpdate
MessageDelete
Notifications
    UserConnected(ModelId)
    UserDisconnected(ModelId)

- Every message sent to the channel must specify who it is directed to
9++
NewMessage -> save to database -> sent listeners -> forward to intended users
UpdateIsRead -> save to database -> sent listeners -> forward to intended users
*/

/// An Incoming message sent by user via websocket
///
/// e.g {"Mgs":{...}}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum IncomingChat {
    #[serde(skip_serializing)]
    NewMessage(NewMessage),
    #[serde(skip_deserializing)]
    DirectMessage(DirectMessage),
    DeleteMessage(DeleteMessage),
    // Notifications
    UpdateMessageRead(UpdateMessagesRead),
    NotifyUserConnected(ModelID),
    NotifyUserDisconnected(ModelID),
}

// ===== New Message impls =====

/// New direct message ws request
#[derive(Debug, Clone, Deserialize)]
pub struct NewMessage {
    pub content: String,
    pub receiver_id: String,
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

impl NewMessageInsertData {
    /// Convert `Self` into `DirectMessage`
    #[must_use]
    pub fn direct_message(self) -> DirectMessage {
        DirectMessage {
            id: self.id,
            sender_id: self.sender_id,
            receiver_id: self.receiver_id,
            content: self.content,
            sent_at: self.sent_at,
            is_author: false,
            is_read: false,
        }
    }
}

/// New Message metadata
#[derive(Debug, Clone)]
pub struct NewMessageStatusInsertData {
    pub message_id: ModelID,
    pub is_read: bool,
    pub sender_has_deleted: bool,
    pub receiver_has_deleted: bool,
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

// ===== Update Message `is_read` impls =====

/// Update messages are `is_read` ws request
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateMessagesRead {
    pub message: Vec<MessageReadUpdate>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageReadUpdate {
    pub sender_id: ModelID,
    pub message_id: ModelID,
}

// ===== Delete Message impls =====

/// Delete direct message ws request
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeleteMessage {
    pub message_id: ModelID,
}
