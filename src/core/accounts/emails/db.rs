//! Email database impls

use time::OffsetDateTime;

use crate::{
    auth::TokenHash,
    endpoint::EndpointRejection,
    error::{ServerError, ServerResult},
    server::state::DatabaseConnection,
    types::ModelID,
};

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
    ) -> ServerResult<Option<(ModelID, String, Option<OffsetDateTime>)>> {
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
            Ok(rec) => Ok(rec.map(|rec| (rec.user_id.into(), rec.email, rec.token_generated_at))),
            Err(err) => {
                tracing::error!("Database error, failed to find email by token: {}", err);
                Err(err.into())
            }
        }
    }

    /// Fetches the user `first_name` and `email` from the database
    #[tracing::instrument(skip(db))]
    pub async fn find_user(
        user_id: ModelID,
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
            user_id.0
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(rec) => Ok((rec.first_name, rec.email)),
            Err(err) => {
                // Handle database constraint error
                handle_email_database_error(&err)?;

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
        user_id: ModelID,
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
            user_id.0,
            values.verified,
            values.email,
            &values.token[..],
            values.token_generated_at,
        )
        .execute(&mut **tx)
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
    pub async fn update(user_id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
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
            user_id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("User email updated successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_email_database_error(&err)?;

                tracing::error!("Database error, failed to update user email: {}", err);
                Err(err.into())
            }
        }
    }

    /// Insert user email into the database
    #[tracing::instrument(skip(db, values), name = "Insert email pending update")]
    pub async fn insert_pending_update(
        user_id: ModelID,
        values: EmailInsertPendingData,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                INSERT INTO accounts.email_pending_updates(
                    id,
                    user_id,
                    new_email, 
                    previous_email_approval_code, 
                    email_change_approved,
                    generated_at
                )
                 VALUES($1, $2, $3, $4, $5, $6)

                ON CONFLICT ON CONSTRAINT email_pending_updates_user_id_key
                DO UPDATE SET new_email = EXCLUDED.new_email,
                    previous_email_approval_code = EXCLUDED.previous_email_approval_code, 
                    new_email_verify_token = NULL,
                    email_change_approved = EXCLUDED.email_change_approved,
                    generated_at = EXCLUDED.generated_at;
              "#,
            values.id.0,
            user_id.0,
            values.new_email,
            &values.previous_email_approval_code[..],
            &values.email_change_approved,
            values.generated_at,
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
                // Handle database constraint error
                handle_email_database_error(&err)?;

                tracing::error!("Database error, failed to verify email: {}", err);
                Err(err.into())
            }
        }
    }

    /// Approve email change into the database
    ///
    /// Return true if the approval code is exists
    #[tracing::instrument(skip(db, approval_hash), name = "Approve email pending update")]
    pub async fn approve_pending_update(
        approval_hash: TokenHash,
        db: DatabaseConnection,
    ) -> ServerResult<Option<String>> {
        match sqlx::query!(
            r#"
                UPDATE accounts.email_pending_updates pending
                    SET email_change_approved = TRUE
                WHERE pending.previous_email_approval_code = $1

                RETURNING pending.new_email
                  "#,
            &approval_hash[..]
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(row) => {
                tracing::debug!("Email change approved");
                Ok(Some(row.new_email))
            }
            Err(err) => {
                if matches!(err, sqlx::Error::RowNotFound) {
                    Ok(None)
                } else {
                    tracing::error!(
                        "Database error, failed to update email pending update approval: {}",
                        err
                    );
                    Err(err.into())
                }
            }
        }
    }

    /// Insert new email verify code in the database
    #[tracing::instrument(skip(user_id, db), name = "Insert new email verify code")]
    pub async fn insert_new_email_verify_code(
        user_id: ModelID,
        verify_hash: TokenHash,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE accounts.email_pending_updates pending
                SET new_email_verify_token = $1
                WHERE pending.user_id = $2
             "#,
            &verify_hash[..],
            user_id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!(
                    "New email verification inserted code successfully: {:?}",
                    result
                );
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_email_database_error(&err)?;

                tracing::error!(
                    "Database error, failed to insert new email verification code: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Verify new email token in the database
    ///
    /// Return true if the token is exists
    #[tracing::instrument(skip(db, verify_hash), name = "Verify new email pending update")]
    pub async fn verify_pending_update(
        verify_hash: TokenHash,
        db: DatabaseConnection,
    ) -> ServerResult<bool> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM accounts.email_pending_updates pending
                    WHERE new_email_verify_token = $1
                        AND email_change_approved = TRUE
                ) AS "exists!"
                  "#,
            &verify_hash[..]
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("New email pending update verified: {:?}", result);
                Ok(result.exists)
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to verify new email pending update: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Deletes email pending updates from the database
    pub async fn delete_pending_updates(
        user_id: ModelID,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                DELETE FROM accounts.email_pending_updates pending
                WHERE pending.id = $1;
            "#,
            user_id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::error!("Email pending updates deleted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to delete email pending updates: {}",
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
                // Handle database constraint error
                handle_email_database_error(&err)?;

                tracing::error!("Database error, failed to delete unverified user: {}", err);
                Err(err.into())
            }
        }
    }
}

/// Handle emails database constraints errors
#[allow(clippy::cognitive_complexity)]
pub fn handle_email_database_error(err: &sqlx::Error) -> ServerResult<()> {
    //
    if matches!(err, &sqlx::Error::RowNotFound) {
        tracing::error!("Database error, Account email not found. {:?}", err);
        return Err(ServerError::rejection(EndpointRejection::NotFound(
            "Account not found.".into(),
        )));
    }

    Ok(())
}
