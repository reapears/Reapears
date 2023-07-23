//! Middleware for authenticating api access

use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use serde::Deserialize;

use crate::{
    auth::{hash_token, TokenHash},
    endpoint::EndpointRejection,
    error::ServerResult,
    server::state::{DatabaseConnection, ServerState},
};

pub mod db;
pub mod handlers;

/// Middleware for authenticating server api access
/// using `api_key` provided in the url
#[derive(Debug, Clone, Copy)]
pub struct ApiAuthentication;

/// Endpoints that are not protected with an API key;
const UNAUTHENTICATED_ENDPOINTS: [&str; 3] = [
    "/account/confirm",
    "/health-check",
    "/account/reset-password",
];

#[async_trait]
impl FromRequestParts<ServerState> for ApiAuthentication {
    type Rejection = EndpointRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        if UNAUTHENTICATED_ENDPOINTS.contains(&parts.uri.path()) {
            return Ok(Self);
        }
        // Extract key from url query.
        let Ok(Query(key)) = Query::<ApiKey>::from_request_parts(parts, state).await else{
            tracing::debug!("Request rejected no api key found");
            return Err(EndpointRejection::unauthorized());
        };
        // Verify api key.
        if !api_token_valid(hash_token(key.api_key.as_bytes()), state.database.clone()).await? {
            tracing::debug!("Request rejected invalid api key.");
            return Err(EndpointRejection::unauthorized());
        }

        Ok(Self)
    }
}

// Helper struct for extracting a key from the url.
#[derive(Deserialize)]
struct ApiKey {
    api_key: String,
}

/// Checks whether the api token is valid
pub async fn api_token_valid(token: TokenHash, db: DatabaseConnection) -> ServerResult<bool> {
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
