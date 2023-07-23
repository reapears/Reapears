//! Password database impls

use time::OffsetDateTime;

use crate::{
    accounts::user::db::handle_user_database_error, auth::TokenHash, error::ServerResult,
    server::state::DatabaseConnection, types::ModelID,
};

use super::PasswordModel;

impl PasswordModel {
    /// Fetches user password from the database
    ///
    /// # Errors
    ///
    /// Return database error
    pub async fn find(user_id: ModelID, db: DatabaseConnection) -> ServerResult<Option<String>> {
        match sqlx::query!(
            r#"
            SELECT user_.phc_string
            FROM accounts.users user_
            WHERE user_.id = $1;
        "#,
            user_id.0
        )
        .fetch_optional(&db.pool)
        .await
        {
            Ok(rec) => Ok(rec.map(|rec| rec.phc_string)),
            Err(err) => {
                tracing::error!("Database error, failed to fetch user password: {}", err);
                Err(err.into())
            }
        }
    }

    /// Updates user password in the database
    pub async fn update(
        user_id: ModelID,
        phc_string: String,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE accounts.users user_
                    SET phc_string = $1
                WHERE user_.id = $2
            
            "#,
            phc_string,
            user_id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(res) => {
                tracing::debug!("User password updated successfully: {:?}", res);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_user_database_error(&err)?;

                tracing::error!("Database error, failed to update user password: {}", err);
                Err(err.into())
            }
        }
    }

    /// Finds find password reset token from the database
    ///
    /// Return (`user_id`, `created_at`) if token found
    pub async fn find_token(
        token: TokenHash,
        db: DatabaseConnection,
    ) -> ServerResult<Option<(ModelID, OffsetDateTime)>> {
        match sqlx::query!(
            r#" 
                SELECT reset_token.user_id,
                    reset_token.token_generated_at
                FROM auth.password_reset_tokens reset_token
                WHERE reset_token.token = $1;
            "#,
            &token[..]
        )
        .fetch_optional(&db.pool)
        .await
        {
            Ok(rec) => Ok(rec.map(|rec| (rec.user_id.into(), rec.token_generated_at))),
            Err(err) => {
                tracing::error!(
                    "Database error, failed to fetch password rest token: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Inserts password reset token into the database
    pub async fn insert_token(
        user_id: ModelID,
        token: TokenHash,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#" 
                INSERT INTO auth.password_reset_tokens(
                    user_id,
                    token,
                    token_generated_at
                )
                VALUES($1, $2, $3)

                ON CONFLICT ON CONSTRAINT password_reset_tokens_pkey
                DO UPDATE SET token = EXCLUDED.token,
                            token_generated_at = EXCLUDED.token_generated_at;
            "#,
            user_id.0,
            &token[..],
            OffsetDateTime::now_utc(),
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Password reset token inserted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to insert password rest token: {}",
                    err
                );
                Err(err.into())
            }
        }
    }
}
