//! Session database impls

use crate::{
    auth::TokenHash, error::ServerResult, server::state::DatabaseConnection, types::ModelID,
};

use super::{
    forms::{SessionInsert, SessionUpdate},
    models::{LoginUser, Session},
    utils::update_last_login,
};

impl Session {
    /// Find the user associated with the email address from the database
    pub async fn find_user_by_email(
        email: String,
        db: DatabaseConnection,
    ) -> ServerResult<Option<LoginUser>> {
        match sqlx::query!(
            r#"
                SELECT user_.id AS user_id,
                    user_.phc_string,
                    user_.account_locked,
                    user_.account_locked_reason,
                    user_.account_locked_until,
                    address.verified AS email_verified,
                    delete_request.requested_at AS "delete_requested_at?"
                FROM accounts.emails address
                LEFT JOIN accounts.users user_
                    ON address.user_id = user_.id
                LEFT JOIN accounts.account_delete_requests delete_request
                    ON address.user_id = delete_request.user_id

                WHERE LOWER(address.email) = LOWER( $1)
            "#,
            email
        )
        .fetch_optional(&db.pool)
        .await
        {
            Ok(rec) => {
                let user = rec.map(|rec| {
                    LoginUser::from_row(
                        rec.user_id.into(),
                        rec.phc_string,
                        rec.account_locked,
                        rec.account_locked_reason,
                        rec.account_locked_until,
                        rec.email_verified,
                        rec.delete_requested_at.is_some(),
                    )
                });
                Ok(user)
            }
            Err(err) => {
                tracing::error!("Database error, failed to fetch LoginUser: {}", err);
                Err(err.into())
            }
        }
    }

    /// Insert a new session into the database.
    #[tracing::instrument(name = "Insert Session", skip(db, session))]
    pub async fn insert(session: SessionInsert, db: DatabaseConnection) -> ServerResult<ModelID> {
        let mut tx = db.pool.begin().await?;
        match sqlx::query!(
            r#"
                INSERT INTO auth.sessions(
                    id, 
                    user_id, 
                    token, 
                    user_agent, 
                    created_at, 
                    last_used_at
                )
                VALUES($1, $2, $3, $4, $5, $6);
            "#,
            session.id.0,
            session.user_id.0,
            &session.token,
            session.user_agent,
            session.created_at,
            session.last_used_at
        )
        .execute(&mut *tx)
        .await
        {
            Ok(result) => {
                // Update user last login date
                update_last_login(session.user_id, &mut tx).await?;

                tx.commit().await?;
                tracing::debug!("Session inserted successfully: {:?}", result);

                Ok(session.id)
            }
            Err(err) => {
                tracing::error!("Database error, failed to insert session: {}", err);
                Err(err.into())
            }
        }
    }

    /// Update session's last_used_at in the database
    #[tracing::instrument(name = "Update Session", skip(db, token, session))]
    pub async fn update(
        token: TokenHash,
        session: SessionUpdate,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE auth.sessions 
                    SET last_used_at = $1
                WHERE sessions.token = $2
            "#,
            session.last_used_at,
            &token[..]
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Session updated successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!("Database error, failed to update session: {}", err);
                Err(err.into())
            }
        }
    }

    /// Delete session from the database
    #[tracing::instrument(name = "Delete Session", skip(db, token))]
    pub async fn delete(token: TokenHash, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                DELETE FROM auth.sessions
                WHERE sessions.token = $1
            "#,
            &token[..]
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Session deleted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!("Database error, failed to delete session: {}", err);
                Err(err.into())
            }
        }
    }
}
