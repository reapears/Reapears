//! Chat Handler
#![allow(dead_code)]
#![allow(clippy::unused_async)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    Json,
};
use futures_util::stream::{SplitSink, SplitStream, StreamExt};

use crate::{
    auth::CurrentUser,
    endpoint::{EndpointRejection, EndpointResult},
    server::state::{DatabaseConnection, ServerState},
};

use super::{
    forms::IncomingChat,
    models::{Conversation, Conversations, DirectMessage},
    ChatBroadcast, ChatMessages,
};

/// Handles the `GET account/users/dms` route.
#[tracing::instrument(skip(db))]
pub async fn user_direct_messages(
    user: CurrentUser,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<Conversations>> {
    Conversations::find(user.id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |conversation| Ok(Json(conversation)),
    )
}

/// jh
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    user: CurrentUser,
    State(state): State<ServerState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| chat_handler(socket, user, state))
}

/// hh
async fn chat_handler(stream: WebSocket, user: CurrentUser, state: ServerState) {
    let (mut outgoing, mut incoming) = stream.split();

    // Announce user online
    let user_id = user.id;
    state.chat.send(IncomingChat::NotifyUserConnected(user_id));

    // Forward messages to user sent via ws
    let mut chat_messages = state.chat.message_listener();
    let mut send_messages = tokio::spawn({
        let user = user.clone();
        async move { send_incoming_messages(user, chat_messages, outgoing).await }
    });

    // Receive messages from via ws
    let chat_broadcast = state.chat.clone();
    let db = state.database.clone();
    let mut recv_messages =
        tokio::spawn(
            async move { recv_incoming_messages(user, chat_broadcast, incoming, db).await },
        );

    tokio::select! {
        _ = (&mut send_messages) => recv_messages.abort(),
        _ = (&mut recv_messages) => send_messages.abort(),
    };

    // Announce user disconnected
    state
        .chat
        .send(IncomingChat::NotifyUserDisconnected(user_id));
}

/// Receive Incoming-messages sent by the user via ws
/// and forward them to connected listeners
#[allow(clippy::unused_async)]
async fn recv_incoming_messages(
    _user: CurrentUser,
    _chat: ChatBroadcast,
    _incoming: SplitStream<WebSocket>,
    _db: DatabaseConnection,
) {
    // while let Some(Ok(Message::Text(msg))) = incoming.next().await {
    //     if let Ok(msg) = serde_json::from_str(&msg) {
    //         match msg {
    //             IncomingChat::NewMessage(msg) => {
    //                 let values = msg.insert_data(user.id);
    //                 let direct_message = values.clone().direct_message();
    //                 if let Ok(_)  = Conversation::insert(values).await{
    //                     chat.send(direct_message);
    //                 }
    //                 // Conversation::
    //             }
    //             IncomingChat::DeleteMessage(req) => {}
    //         }

    //         let incoming_msg: IncomingChat = serde_json::from_str(&msg);
    //         // save to the database and broadcast it
    //         chat.send(incoming_msg);
    //     }
    // }
}

/// Send Incoming-messages to the user via ws
#[allow(clippy::unused_async)]
async fn send_incoming_messages(
    _user: CurrentUser,
    _messages: ChatMessages,
    _outgoing: SplitSink<WebSocket, Message>,
) {
    // todo!();
    // while let Ok(msg) = messages.recv().await {
    //     // on message just check if the message send to me
    //     let msg = serde_json::to_string(msg).unwrap();
    //     if outgoing.send(Message::Text(msg)).await.is_err() {
    //         break;
    //     }
    // }
}
