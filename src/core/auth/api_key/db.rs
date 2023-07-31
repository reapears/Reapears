//! Api key database impl

use crate::{
    auth::TokenHash,
    endpoint::EndpointRejection,
    error::{ServerError, ServerResult},
    server::state::DatabaseConnection,
    types::ModelID,
};

use super::{ApiToken, ApiTokenList};

impl ApiToken {
    /// Checks whether the api token exists in the database.
    pub async fn exists(token: TokenHash, db: DatabaseConnection) -> ServerResult<bool> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM auth.api_tokens
                    WHERE token = $1 AND revoked = FALSE
                ) AS "is_valid!"
        "#,
            &token[..]
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(row) => Ok(row.is_valid),
            Err(err) => {
                tracing::error!("Database error, failed to fetch current-user: {}", err);
                Err(err.into())
            }
        }
    }

    /// Fetches Api token records from the database
    #[tracing::instrument(name = "Fetch Api tokens", skip(db))]
    pub async fn records(db: DatabaseConnection) -> ServerResult<ApiTokenList> {
        match sqlx::query!(
            r#"
                SELECT token.id,
                     token.user_id,
                     token.token,
                     token.belongs_to,
                     token.created_at,
                     token.last_used_at,
                     token.revoked
                FROM auth.api_tokens token
            "#
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let api_tokens = records
                    .into_iter()
                    .map(|rec| {
                        Self::from_row(
                            rec.id.into(),
                            rec.user_id.map(Into::into),
                            rec.token,
                            rec.belongs_to,
                            rec.created_at,
                            rec.last_used_at,
                            rec.revoked,
                        )
                    })
                    .collect();

                Ok(api_tokens)
            }
            Err(err) => {
                tracing::error!("Database error, failed to fetch Api tokens: {}", err);
                Err(err.into())
            }
        }
    }

    /// Fetches Api token from the database
    #[tracing::instrument(name = "Find Api token", skip(db, token))]
    pub async fn find(token: TokenHash, db: DatabaseConnection) -> ServerResult<Option<Self>> {
        match sqlx::query!(
            r#"
                SELECT token.id,
                     token.user_id,
                     token.token,
                     token.belongs_to,
                     token.created_at,
                     token.last_used_at,
                     token.revoked
                FROM auth.api_tokens token

                WHERE token.token = $1 AND token.revoked = FALSE
            "#,
            &token[..]
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(rec) => {
                let api_token = Self::from_row(
                    rec.id.into(),
                    rec.user_id.map(Into::into),
                    rec.token,
                    rec.belongs_to,
                    rec.created_at,
                    rec.last_used_at,
                    rec.revoked,
                );

                Ok(Some(api_token))
            }
            Err(err) => {
                if matches!(err, sqlx::Error::RowNotFound) {
                    Ok(None)
                } else {
                    tracing::error!("Database error, failed to fetch Api token: {}", err);
                    Err(err.into())
                }
            }
        }
    }

    /// Inserts Api token into the database
    #[tracing::instrument(name = "Insert Api token", skip(db, self))]
    pub async fn insert(self, db: DatabaseConnection) -> ServerResult<Vec<u8>> {
        match sqlx::query!(
            r#"
                INSERT INTO auth.api_tokens(
                    id,
                    user_id,
                    token,
                    belongs_to,
                    created_at,
                    last_used_at,
                    revoked
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)

                --ON CONFLICT ON CONSTRAINT api_tokens_user_id_fkey
                --DO UPDATE SET token = EXCLUDED.token,
                --            belongs_to = EXCLUDED.belongs_to,
                 --           created_at = EXCLUDED.created_at,
                 --           last_used_at = EXCLUDED.last_used_at,
                 --           revoked = EXCLUDED.revoked;
            "#,
            self.id.0,
            self.user_id.map(|id| id.0),
            &self.token[..],
            self.belongs_to,
            self.created_at,
            self.last_used_at,
            self.revoked
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Api token inserted successfully: {:?}", result);
                Ok(self.token)
            }
            Err(err) => {
                // Handle database constraint error
                handle_api_token_database_error(&err)?;

                tracing::error!("Database error, failed to insert Api token: {}", err);
                Err(err.into())
            }
        }
    }

    /// Update Api token `last_access_at` field in the database
    #[tracing::instrument(name = "Delete Api token", skip(db, token))]
    pub async fn update_last_access_at(
        token: TokenHash,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                DELETE FROM auth.api_tokens token
                WHERE token.token = $1
           "#,
            &token[..]
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Api token deleted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_api_token_database_error(&err)?;

                tracing::error!("Database error, failed to delete Api token: {}", err);
                Err(err.into())
            }
        }
    }

    /// Deletes Api token from the database
    #[tracing::instrument(name = "Delete Api token", skip(db, token))]
    pub async fn delete_by_hash(token: TokenHash, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                DELETE FROM auth.api_tokens token
                WHERE token.token = $1
           "#,
            &token[..]
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Api token deleted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_api_token_database_error(&err)?;

                tracing::error!("Database error, failed to delete Api token: {}", err);
                Err(err.into())
            }
        }
    }

    /// Deletes Api token from the database
    #[tracing::instrument(name = "Delete Api token", skip(db))]
    pub async fn delete_by_id(id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                 DELETE FROM auth.api_tokens token
                 WHERE token.id = $1
            "#,
            id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Api token deleted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_api_token_database_error(&err)?;

                tracing::error!("Database error, failed to delete Api token: {}", err);
                Err(err.into())
            }
        }
    }
}

/// Handle regions database constraints errors
#[allow(clippy::cognitive_complexity)]
fn handle_api_token_database_error(err: &sqlx::Error) -> ServerResult<()> {
    if let sqlx::Error::Database(db_err) = err {
        // Handle db unique constraints
        if db_err.is_unique_violation() {
            tracing::error!("Database error,Token already exists. {:?}", err);
            return Err(ServerError::rejection(EndpointRejection::Conflict(
                "Try requesting a new token again.".into(),
            )));
        }

        // Handle db foreign key constraints
        if db_err.is_foreign_key_violation() {
            tracing::error!("Database error, user not found. {:?}", err);
            return Err(ServerError::rejection(EndpointRejection::BadRequest(
                "User  not found.".into(),
            )));
        }
    }

    if matches!(err, &sqlx::Error::RowNotFound) {
        tracing::error!("Database error, token not found. {:?}", err);
        return Err(ServerError::rejection(EndpointRejection::NotFound(
            "Token not found.".into(),
        )));
    }

    Ok(())
}
