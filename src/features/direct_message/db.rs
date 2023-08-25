//! Direct Message database impl

use itertools::Itertools;
use time::OffsetDateTime;

use crate::{
    endpoint::EndpointRejection,
    error::{ServerError, ServerResult},
    server::state::DatabaseConnection,
    types::ModelID,
};

use super::{
    forms::NewMessageInsertData,
    models::{Conversation, Conversations, DirectMessage},
    utils::{find_message_status, insert_message_status, message_receiver_count},
};

impl Conversations {
    /// Fetches all conversations of this user had from the database
    #[tracing::instrument(name = "Fetch all conversations", skip(db))]
    pub async fn find(user_id: ModelID, db: DatabaseConnection) -> ServerResult<Self> {
        match sqlx::query!(
            r#"
            SELECT message.id AS "message_id!",
                message.sender_id AS "sender_id!",
                message.receiver_id AS "receiver_id!", 
                message.content AS "message_content!",
                message.sent_at AS "message_sent_at!",
                status.is_read AS is_read,
                status.sender_has_deleted AS sender_has_deleted,
                status.receiver_has_deleted AS receiver_has_deleted
            FROM features.direct_messages message
            LEFT JOIN features.message_status status
                ON message.id = status.message_id

            WHERE message.sender_id = $1 OR message.receiver_id = $1;
        "#,
            user_id.0,
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let mut conversations = Vec::new();
                for (participant_id, conversation) in &records.into_iter().group_by(|msg| {
                    if msg.sender_id == user_id {
                        msg.receiver_id
                    } else {
                        msg.sender_id
                    }
                }) {
                    let messages: Vec<_> = conversation
                        .into_iter()
                        .filter(|rec| {
                            // Filter deleted messages
                            (rec.sender_id == user_id && !rec.sender_has_deleted)
                                || (rec.receiver_id == user_id && !rec.receiver_has_deleted)
                        })
                        .map(|rec| {
                            let sender_id: ModelID = rec.sender_id.into();
                            DirectMessage::from_row(
                                rec.message_id.into(),
                                sender_id,
                                rec.receiver_id.into(),
                                rec.message_content,
                                rec.message_sent_at,
                                sender_id == user_id,
                                rec.is_read,
                            )
                        })
                        .collect();

                    conversations.push(Conversation::from_row(
                        user_id,
                        participant_id.into(),
                        messages,
                    ));
                }

                Ok(Self::from_row(conversations))
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to fetch all user conversations: {}",
                    err
                );
                Err(err.into())
            }
        }
    }
}

// ===== Conversation impls =====

impl Conversation {
    /// Fetches conversation from the database
    #[tracing::instrument(name = "Fetch Conversation", skip(db))]
    pub async fn find(
        user_id: ModelID,
        other_id: ModelID,
        db: DatabaseConnection,
    ) -> ServerResult<Self> {
        match sqlx::query!(
            r#"
                SELECT message.id AS "message_id!",
                    message.sender_id AS "sender_id!",
                    message.receiver_id AS "receiver_id!", 
                    message.content AS "message_content!",
                    message.sent_at AS "message_sent_at!",
                    status.is_read AS is_read,
                    status.sender_has_deleted AS sender_has_deleted,
                    status.receiver_has_deleted AS receiver_has_deleted
                FROM features.direct_messages message
                LEFT JOIN features.message_status status
                    ON message.id = status.message_id

                WHERE (message.sender_id = $1 AND message.receiver_id = $2) OR
                        (message.sender_id = $2 AND message.receiver_id = $1)
            "#,
            user_id.0,
            other_id.0
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let conversation: Vec<_> = records
                    .into_iter()
                    .filter(|rec| {
                        // Filter deleted messages
                        (rec.sender_id == user_id && !rec.sender_has_deleted)
                            || (rec.receiver_id == user_id && !rec.receiver_has_deleted)
                    })
                    .map(|rec| {
                        let sender_id: ModelID = rec.sender_id.into();
                        DirectMessage::from_row(
                            rec.message_id.into(),
                            sender_id,
                            rec.receiver_id.into(),
                            rec.message_content,
                            rec.message_sent_at,
                            sender_id == user_id,
                            rec.is_read,
                        )
                    })
                    .collect();

                Ok(Self::from_row(user_id, other_id, conversation))
            }
            Err(err) => {
                tracing::error!("Database error, failed to fetch conversation: {}", err);
                Err(err.into())
            }
        }
    }

    /// Inserts Direct Message into the database
    #[tracing::instrument(name = "Insert Direct Message", skip(db, msg))]
    pub async fn insert(
        msg: NewMessageInsertData,
        db: DatabaseConnection,
    ) -> ServerResult<ModelID> {
        let mut tx = db.pool.begin().await?; // init transaction
        match sqlx::query!(
            r#"
                INSERT INTO features.direct_messages(
                    id,
                    sender_id,
                    receiver_id,
                    content,
                    sent_at
                )
                VALUES($1, $2, $3, $4, $5);
            "#,
            msg.id.0,
            msg.sender_id.0,
            msg.receiver_id.0,
            msg.content,
            msg.sent_at,
        )
        .execute(&mut *tx)
        .await
        {
            Ok(result) => {
                tracing::debug!(
                    "Direct Message inserted successfully, but transaction not committed: {:?}",
                    result
                );
                // Insert direct message metadata
                insert_message_status(msg.status, &mut tx).await?;

                tx.commit().await?; // Commit transaction
                tracing::debug!("Direct message and its metadata inserted successfully.");
                Ok(msg.id)
            }
            Err(err) => {
                tracing::error!("Database error, failed to insert direct message: {}", err);
                Err(err.into())
            }
        }
    }

    /// Deletes Direct Message from the database
    #[tracing::instrument(name = "Delete Direct Message", skip(db))]
    pub async fn delete(
        user_id: ModelID,
        message_id: ModelID,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        let msg_status = find_message_status(message_id, db.clone()).await?;

        let is_sender = user_id == msg_status.sender_id;
        let is_receiver = user_id == msg_status.receiver_id;

        if !(is_sender || is_receiver) {
            return Err(ServerError::rejection(EndpointRejection::forbidden()));
        }

        // If of the user already deleted this message; delete it permanently.
        if (is_sender && msg_status.receiver_has_deleted)
            || (is_receiver && msg_status.sender_has_deleted)
        {
            DirectMessage::delete(message_id, db).await
        }
        // Else delete it for only this user
        else if is_sender {
            DirectMessage::delete_for_sender(message_id, db).await
        } else {
            DirectMessage::delete_for_receiver(message_id, db).await
        }
    }

    /// Deletes Direct Message from the database for everyone
    #[tracing::instrument(name = "Delete direct message for everyone", skip(db))]
    pub async fn delete_for_everyone(
        message_id: ModelID,
        user_id: ModelID,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        let msg_status = find_message_status(message_id, db.clone()).await?;

        let is_sender = user_id == msg_status.sender_id;

        if !is_sender {
            return Err(ServerError::rejection(EndpointRejection::forbidden()));
        }
        DirectMessage::delete(message_id, db).await
    }

    /// Updates the direct message is_read in the database
    #[tracing::instrument(name = "Update messages are read", skip(db))]
    #[allow(clippy::cast_possible_wrap)]
    pub async fn update_is_read(
        receiver_id: ModelID,
        message_ids: Vec<ModelID>,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        let message_ids: Vec<_> = message_ids.into_iter().map(|id| id.0).collect();
        let db_count = message_receiver_count(receiver_id, message_ids.clone(), db.clone()).await?;

        if Some(message_ids.len() as i64) != db_count {
            return Err(ServerError::rejection(EndpointRejection::forbidden()));
        }
        // Update messages
        match sqlx::query!(
            r#"
                UPDATE features.message_status status
                    SET is_read = TRUE,
                        read_at = $1
                WHERE status.message_id = ANY($2);
            "#,
            OffsetDateTime::now_utc(),
            &message_ids[..],
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::trace!(
                    "Direct message is_read status successfully updated: {:?}",
                    result
                );
                Ok(())
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to update direct message is_read status: {}",
                    err
                );
                Err(err.into())
            }
        }
    }
}

// ===== Direct Message impls =====

impl DirectMessage {
    /// Updates direct message is read in the database
    #[allow(dead_code)]
    async fn update_is_read(message_ids: Vec<ModelID>, db: DatabaseConnection) -> ServerResult<()> {
        let message_ids: Vec<_> = message_ids.into_iter().map(|id| id.0).collect();
        match sqlx::query!(
            r#"
                UPDATE features.message_status status
                    SET is_read = TRUE,
                        read_at = $1
                WHERE status.message_id = ANY($2);
            "#,
            OffsetDateTime::now_utc(),
            &message_ids[..],
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::trace!(
                    "Direct message is_read status successfully updated: {:?}",
                    result
                );
                Ok(())
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to update direct message is_read status: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Permanently delete direct message from the database
    async fn delete(message_id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                DELETE FROM features.message_status status
                WHERE status.message_id = $1
            "#,
            message_id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::trace!("Direct message successfully deleted: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!("Database error, failed to delete direct message: {}", err);
                Err(err.into())
            }
        }
    }

    /// Deleted direct message for this `sender` in the database
    async fn delete_for_sender(message_id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE features.message_status status
                    SET sender_has_deleted = TRUE,
                    sender_deleted_at = $1
                WHERE status.message_id = $2;
            "#,
            OffsetDateTime::now_utc(),
            message_id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::trace!(
                    "Direct message successfully deleted for sender: {:?}",
                    result
                );
                Ok(())
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to delete direct message for sender: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Deleted direct message for this `receiver` in the database
    async fn delete_for_receiver(message_id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE features.message_status status
                    SET receiver_has_deleted = TRUE,
                    receiver_deleted_at = $1
                WHERE status.message_id = $2;
            "#,
            OffsetDateTime::now_utc(),
            message_id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::trace!(
                    "Direct message successfully deleted for receiver: {:?}",
                    result
                );
                Ok(())
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to delete direct message for receiver: {}",
                    err
                );
                Err(err.into())
            }
        }
    }
}
