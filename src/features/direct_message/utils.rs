//! Direct Message helpers impls

use crate::{error::ServerResult, server::state::DatabaseConnection, types::ModelID};

use super::forms::NewMessageStatusInsertData;

/// Insert message status into the database
pub async fn insert_message_status(
    status: NewMessageStatusInsertData,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<()> {
    match sqlx::query!(
        r#"
            INSERT INTO features.message_status(
                message_id,
                is_read,
                sender_has_deleted,
                receiver_has_deleted
            )
            VALUES($1, $2, $3, $4);
        "#,
        status.message_id.0,
        status.is_read,
        status.sender_has_deleted,
        status.receiver_has_deleted
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "Message status successfully inserted, but transaction not committed: {:?}",
                result
            );
            Ok(())
        }
        Err(err) => {
            tracing::error!("Database error, failed to insert Message status: {}", err);
            Err(err.into())
        }
    }
}

/// Fetch message delete status from database
pub async fn find_message_status(
    message_id: ModelID,
    db: DatabaseConnection,
) -> ServerResult<MessageDeleteStatus> {
    match sqlx::query!(
        r#"
            SELECT message.sender_id,
                message.receiver_id,
                status.sender_has_deleted,
                status.receiver_has_deleted
            FROM features.direct_messages message
            LEFT JOIN features.message_status status
                on message.id = status.message_id

            WHERE message.id = $1;
        "#,
        message_id.0
    )
    .fetch_one(&db.pool)
    .await
    {
        Ok(rec) => Ok(MessageDeleteStatus::from_row(
            rec.sender_id.into(),
            rec.receiver_id.into(),
            rec.sender_has_deleted,
            rec.receiver_has_deleted,
        )),
        Err(err) => {
            tracing::error!(
                "Database error, failed to fetch message delete status: {}",
                err
            );
            Err(err.into())
        }
    }
}

/// Helper struct used for carrying information
/// for whether the message can be deleted
#[derive(Debug, Clone)]
pub struct MessageDeleteStatus {
    pub sender_id: ModelID,
    pub receiver_id: ModelID,
    pub sender_has_deleted: bool,
    pub receiver_has_deleted: bool,
}

impl MessageDeleteStatus {
    /// Creates a new `MessageDeleteStatus` from the database row
    pub fn from_row(
        sender_id: ModelID,
        receiver_id: ModelID,
        sender_has_deleted: bool,
        receiver_has_deleted: bool,
    ) -> Self {
        Self {
            sender_id,
            receiver_id,
            sender_has_deleted,
            receiver_has_deleted,
        }
    }
}

/// Helper function to verify the user is the receiver
/// of all the message they want to update to `is_read`.
///
/// Return the count of message where they are receivers
/// and the `message.id` is in the `message_ids` they want to update
pub async fn message_receiver_count(
    receiver_id: ModelID,
    message_ids: Vec<uuid::Uuid>,
    db: DatabaseConnection,
) -> ServerResult<Option<i64>> {
    match sqlx::query!(
        r#"
            SELECT COUNT(message.id) AS message_count
            FROM features.direct_messages message
            WHERE (
                message.receiver_id = $1 
                AND
               message.id = ANY($2)
            );
        "#,
        receiver_id.0,
        &message_ids[..],
    )
    .fetch_one(&db.pool)
    .await
    {
        Ok(rec) => Ok(rec.message_count),
        Err(err) => {
            tracing::error!(
                "Database error, failed to fetch message count for is_read update: {}",
                err
            );
            Err(err.into())
        }
    }
}
