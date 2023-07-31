//! Chat system impls

use std::sync::Arc;

use tokio::sync::broadcast;

use crate::{
    error::{ServerError, ServerResult},
    types::ModelID,
};

use super::{
    forms::{ForwardMessage, IncomingMessageError, MessageIsRead},
    models::DirectMessage,
};

/// Broadcasts `BroadcastMessage` with subscribed users.
#[derive(Debug, Clone)]
pub struct ChatFeed(Arc<broadcast::Sender<BroadcastMessage>>);

impl Default for ChatFeed {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatFeed {
    /// Creates a new `ChatFeed`
    #[must_use]
    pub fn new() -> Self {
        let (sender, _recv) = broadcast::channel(1024);
        Self(Arc::new(sender))
    }

    /// Sends a `BroadcastMessage` to all subscribed message listeners.
    pub fn broadcast(&self, msg: BroadcastMessage) {
        let _ = self.0.send(msg);
    }

    /// Returns an new `MessageListener`
    #[must_use]
    pub fn subscribe(&self) -> MessageListener {
        MessageListener(self.0.subscribe())
    }
}

// ===== MessageListener impls =====

/// Listens for `BroadcastMessage` sent on the queue.
#[derive(Debug)]
pub struct MessageListener(broadcast::Receiver<BroadcastMessage>);

impl MessageListener {
    /// Listens for `BroadcastMessage` sent on the queue.
    pub async fn listen(&mut self) -> ServerResult<BroadcastMessage> {
        self.0
            .recv()
            .await
            .map_err(|err| ServerError::new(err.to_string()))
    }
}

// ===== BroadcastMessage impls =====

/// Message send on message queue.
#[derive(Debug, Clone)]
pub struct BroadcastMessage {
    pub forward_to: MessageForwardTo,
    message: ForwardMessage,
}

/// Marks the message to whom it should be forwarded to.
#[derive(Debug, Clone)]
pub enum MessageForwardTo {
    UserId(ModelID),
    ForAll,
}

impl BroadcastMessage {
    /// Returns message if it is intended for this user.
    #[must_use]
    pub fn message(self, user_id: ModelID) -> Option<ForwardMessage> {
        if self.is_for_me(user_id) {
            Some(self.message)
        } else {
            None
        }
    }

    /// Return whether the message is intended for this user.
    #[must_use]
    pub fn is_for_me(&self, user_id: ModelID) -> bool {
        match self.forward_to {
            MessageForwardTo::UserId(id) => id == user_id,
            MessageForwardTo::ForAll => true,
        }
    }

    /// Creates `UserConnected` forward message.
    #[must_use]
    pub fn user_connected(user_id: ModelID) -> Self {
        Self::for_all(ForwardMessage::UserConnected(user_id))
    }

    /// Creates `UserDisconnected` forward message.
    #[must_use]
    pub fn user_disconnected(user_id: ModelID) -> Self {
        Self::for_all(ForwardMessage::UserDisconnected(user_id))
    }

    /// Creates `DirectMessage` forward message.
    #[must_use]
    pub fn direct_message(to: ModelID, message: DirectMessage) -> Self {
        Self::for_user(to, ForwardMessage::DirectMessage(message))
    }

    /// Creates `MessageIsRead` forward message.
    #[must_use]
    pub fn message_is_read(to: ModelID, message: MessageIsRead) -> Self {
        Self::for_user(to, ForwardMessage::MessageIsRead(message))
    }

    /// Creates `IncomingMessageError` forward message.
    #[must_use]
    pub fn message_error(to: ModelID, err: IncomingMessageError) -> Self {
        Self::for_user(to, ForwardMessage::IncomingMessageError(err))
    }

    /// Creates a `BroadcastMessage` that is sent this user.
    #[must_use]
    pub fn for_user(user_id: ModelID, message: ForwardMessage) -> Self {
        Self {
            forward_to: MessageForwardTo::UserId(user_id),
            message,
        }
    }

    /// Creates a `BroadcastMessage` that is sent to all connected users.
    #[must_use]
    pub fn for_all(message: ForwardMessage) -> Self {
        Self {
            forward_to: MessageForwardTo::ForAll,
            message,
        }
    }
}
