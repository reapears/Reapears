//! `DirectMessage` system impls

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    Json,
};
use futures_util::{
    stream::{SplitSink, SplitStream, StreamExt},
    SinkExt,
};

use crate::{
    auth::CurrentUser,
    endpoint::{EndpointRejection, EndpointResult},
    server::state::{DatabaseConnection, ServerState},
};

use super::{
    forms::{IncomingMessage, IncomingMessageError},
    models::{Conversation, Conversations},
    BroadcastMessage, ChatFeed, MessageListener,
};

/// Handles the `GET account/users/dms` route.
#[tracing::instrument(skip(db))]
pub async fn user_conversations(
    user: CurrentUser,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<Conversations>> {
    Conversations::find(user.id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |conversation| Ok(Json(conversation)),
    )
}

/// Sets up direct message chat system.
#[allow(clippy::unused_async)]
pub async fn direct_message_websocket(
    ws: WebSocketUpgrade,
    user: CurrentUser,
    State(state): State<ServerState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| direct_message_handler(socket, user, state))
}

/// Manages connected user chat.
async fn direct_message_handler(stream: WebSocket, user: CurrentUser, state: ServerState) {
    let user_id = user.id;
    let chat = state.chat_feed();
    let (outgoing, incoming) = stream.split();

    // Broadcast user connected
    chat.broadcast(BroadcastMessage::user_connected(user_id));

    // Listens for message sent in the message queue
    // and forward them to user if it is intended for them.
    let mut send_messages = tokio::spawn({
        let user = user.clone();
        let message_listener = state.chat_feed_as_ref().subscribe();
        async move { listen_send_messages(user, message_listener, outgoing).await }
    });

    // Receive incoming messages sent by user via websocket
    // and process them.
    let mut recv_messages = tokio::spawn({
        let chat = state.chat_feed();
        let db = state.database();
        async move { recv_broadcast_messages(user, chat, incoming, db).await }
    });

    tokio::select! {
        _ = (&mut send_messages) => recv_messages.abort(),
        _ = (&mut recv_messages) => send_messages.abort(),
    };

    // Broadcast user disconnected
    chat.broadcast(BroadcastMessage::user_disconnected(user_id));
}

/// Listens for `BroadcastsMessage`s sent in message queue
/// and forward them user via ws if it is intended for them.
async fn listen_send_messages(
    user: CurrentUser,
    mut messages: MessageListener,
    mut outgoing: SplitSink<WebSocket, Message>,
) {
    while let Ok(msg) = messages.listen().await {
        // Forward the message if it is intended for this user.
        if let Some(forward_msg) = msg.message(user.id) {
            let forward_msg = serde_json::to_string(&forward_msg).unwrap();
            if outgoing.send(Message::Text(forward_msg)).await.is_err() {
                break;
            }
        }
    }
}

/// Receive `IncomingMessage`s sent by the user via ws
/// and broadcast them to subscribed message listeners.
async fn recv_broadcast_messages(
    user: CurrentUser,
    chat: ChatFeed,
    mut incoming: SplitStream<WebSocket>,
    db: DatabaseConnection,
) {
    // Listens for incoming message and process them.
    while let Some(Ok(Message::Text(msg))) = incoming.next().await {
        let msg_result: Result<IncomingMessage, _> = serde_json::from_str(&msg);
        match msg_result {
            Ok(msg) => process_incoming_message(user.clone(), msg, chat.clone(), db.clone()).await,
            Err(err) => {
                tracing::error!("IncomingMessage deserialization error: {:?}", err);
                chat.broadcast(BroadcastMessage::message_error(
                    user.id,
                    IncomingMessageError::UnprocessableEntity,
                ));
            }
        }
    }
}

// Process IncomingMessages
async fn process_incoming_message(
    user: CurrentUser,
    msg: IncomingMessage,
    chat: ChatFeed,
    db: DatabaseConnection,
) {
    match msg {
        IncomingMessage::NewMessage(new_msg) => {
            let insert_data = new_msg.insert_data(user.id);
            let direct_msg = insert_data.direct_message();
            match Conversation::insert(insert_data, db).await {
                Ok(_) => chat.broadcast(BroadcastMessage::direct_message(
                    direct_msg.receiver_id,
                    direct_msg,
                )),
                Err(_err) => {
                    chat.broadcast(BroadcastMessage::message_error(
                        user.id,
                        IncomingMessageError::InternalServerError,
                    ));
                }
            }
        }

        IncomingMessage::MessageIsRead(msg_read_update) => {
            let message_ids = msg_read_update.messages.clone();
            match Conversation::update_is_read(user.id, message_ids, db).await {
                Ok(()) => chat.broadcast(BroadcastMessage::message_is_read(
                    msg_read_update.sender_id,
                    msg_read_update,
                )),
                Err(_err) => {
                    chat.broadcast(BroadcastMessage::message_error(
                        user.id,
                        IncomingMessageError::InternalServerError,
                    ));
                }
            }
        }

        IncomingMessage::MessageDelete(msg) => {
            match Conversation::delete(user.id, msg.message_id, db).await {
                Ok(()) => {}
                Err(_err) => {
                    chat.broadcast(BroadcastMessage::message_error(
                        user.id,
                        IncomingMessageError::InternalServerError,
                    ));
                }
            }
        }

        IncomingMessage::UserConnected => {
            chat.broadcast(BroadcastMessage::user_connected(user.id));
        }

        IncomingMessage::UserDisconnected => {
            chat.broadcast(BroadcastMessage::user_disconnected(user.id));
        }
    }
}
