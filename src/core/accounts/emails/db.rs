//! Email database impls

use time::OffsetDateTime;
use uuid::Uuid;

use crate::{auth::TokenHash, error::ServerResult, server::state::DatabaseConnection};

use super::{
    forms::{EmailInsertData, EmailInsertPendingData},
    EmailModel,
};

impl EmailModel {
    /// Find the email associated with the token from the database
    #[tracing::instrument(skip(db, token))]
    pub async fn find_by_token(
        token: TokenHash,
        db: DatabaseConnection,
    ) -> ServerResult<Option<(Uuid, String, Option<OffsetDateTime>)>> {
        match sqlx::query!(
            r#"
                SELECT user_id,
                    email,
                    token_generated_at
                FROM accounts.emails address
                WHERE address.token = $1
                    AND verified = FALSE;
            "#,
            &token[..]
        )
        .fetch_optional(&db.pool)
        .await
        {
            Ok(rec) => Ok(rec.map(|rec| (rec.user_id, rec.email, rec.token_generated_at))),
            Err(err) => {
                tracing::error!("Database error, failed to find email by token: {}", err);
                Err(err.into())
            }
        }
    }

    /// Fetches the user `first_name` and `email` from the database
    #[tracing::instrument(skip(db))]
    pub async fn find_user(
        user_id: Uuid,
        db: DatabaseConnection,
    ) -> ServerResult<(String, String)> {
        match sqlx::query!(
            r#"
                SELECT user_.first_name,
                    address.email
                FROM accounts.users user_
                LEFT JOIN accounts.emails address
                    ON user_.id = address.user_id
                WHERE user_.id = $1
            "#,
            user_id
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(rec) => Ok((rec.first_name, rec.email)),
            Err(err) => {
                tracing::error!("Database error, failed to find email by token: {}", err);
                Err(err.into())
            }
        }
    }

    /// Checks if the email exists and verified in the database
    #[tracing::instrument(skip(db, email), name = "Check verified email exists")]
    pub async fn exists_and_verified(email: String, db: DatabaseConnection) -> ServerResult<bool> {
        Self::exists(email, true, db).await
    }

    /// Checks if the email exists and unverified in the database
    #[tracing::instrument(skip(db, email), name = "Check unverified email exists")]
    pub async fn exists_and_unverified(
        email: String,
        db: DatabaseConnection,
    ) -> ServerResult<bool> {
        Self::exists(email, false, db).await
    }

    /// Checks if the email exists in the database
    async fn exists(email: String, verified: bool, db: DatabaseConnection) -> ServerResult<bool> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM accounts.emails address
                    WHERE LOWER(address.email) = LOWER($1)
                        AND address.verified = $2
                ) AS "exists!"
            "#,
            email,
            verified
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(result) => Ok(result.exists),
            Err(err) => {
                tracing::error!("Database error, failed to check if email exists: {}", err);
                Err(err.into())
            }
        }
    }

    /// Insert user email into the database
    #[tracing::instrument(skip(tx, values), name = "Insert Email")]
    pub async fn insert(
        user_id: Uuid,
        values: EmailInsertData,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                INSERT INTO accounts.emails(
                    user_id, 
                    verified, 
                    email, 
                    token, 
                    token_generated_at
                )
                 VALUES($1, $2, $3, $4, $5);
              "#,
            user_id,
            values.verified,
            values.email,
            &values.token[..],
            values.token_generated_at,
        )
        .execute(tx)
        .await
        {
            Ok(result) => {
                tracing::debug!(
                    "User email inserted successfully, but transaction not committed: {:?}",
                    result
                );
                Ok(())
            }
            Err(err) => {
                tracing::error!("Database error, failed to insert user email: {}", err);
                Err(err.into())
            }
        }
    }

    /// Updates user email in the database
    #[tracing::instrument(skip(user_id, db), name = "Update Email")]
    pub async fn update(user_id: Uuid, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE accounts.emails AS address
                SET email = (
                        SELECT new_email 
                        FROM accounts.email_pending_updates 
                        WHERE user_id = $1
                    ),
                    verified = true
                    
                WHERE user_id = $1;
            "#,
            user_id
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("User email updated successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!("Database error, failed to update user email: {}", err);
                Err(err.into())
            }
        }
    }

    /// Insert user email into the database
    #[tracing::instrument(skip(db, values), name = "Insert email pending update")]
    pub async fn insert_pending_update(
        user_id: Uuid,
        values: EmailInsertPendingData,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                INSERT INTO accounts.email_pending_updates(
                    id,
                    user_id,
                    new_email, 
                    token, 
                    token_generated_at
                )
                 VALUES($1, $2, $3, $4, $5);
              "#,
            values.id,
            user_id,
            values.new_email,
            &values.token[..],
            values.token_generated_at,
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!(
                    "User email pending update successfully inserted: {:?}",
                    result
                );
                Ok(())
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to insert user email pending update: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Updates the email address field verified=true
    #[tracing::instrument(skip(db, email))]
    pub async fn verify(email: String, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE accounts.emails address
                SET verified = TRUE,
                    token = NULL,
                    token_generated_at = NULL
                WHERE address.email = $1
            "#,
            email
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Account confirmed successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!("Database error, failed to verify email: {}", err);
                Err(err.into())
            }
        }
    }

    /// Checks if the email pending update exists in the database
    #[tracing::instrument(skip(db, user_id, code), name = "Check email update exists")]
    pub async fn pending_update_exists(
        user_id: Uuid,
        code: TokenHash,
        db: DatabaseConnection,
    ) -> ServerResult<bool> {
        match sqlx::query!(
            r#"
                 SELECT EXISTS(
                     SELECT 1 FROM accounts.email_pending_updates pending
                     WHERE pending.user_id = $1
                        AND pending.token = $2
                 ) AS "exists!"
             "#,
            user_id,
            &code
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(result) => Ok(result.exists),
            Err(err) => {
                tracing::error!(
                    "Database error, failed to check if email pending update exists: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Deletes unconfirmed user account from the database
    pub async fn delete_unverified(email: String, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                DELETE FROM accounts.users user_
                WHERE user_.id IN (
                    SELECT address.user_id
                    FROM accounts.emails address
                    WHERE address.email = $1
                )
            "#,
            email
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::error!("Unverified user deleted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!("Database error, failed to delete unverified user: {}", err);
                Err(err.into())
            }
        }
    }
}
