//! Session helpers impls

use time::OffsetDateTime;

use crate::{error::ServerResult, types::ModelID};

/// Update user last login date
///
/// # Errors
///
/// Return database error
pub async fn update_last_login(
    user_id: ModelID,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<()> {
    match sqlx::query!(
        r#"
            UPDATE accounts.users user_
                SET last_login = $1
            
            WHERE user_.id = $2;
        "#,
        OffsetDateTime::now_utc(),
        user_id.0
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "User last login date updated successfully, but transaction not committed: {:?}",
                result
            );
            Ok(())
        }
        Err(err) => {
            tracing::error!(
                "Database error, failed to update user last login date: {}",
                err
            );
            Err(err.into())
        }
    }
}
