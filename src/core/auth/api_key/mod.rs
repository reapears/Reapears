//! Middleware for authenticating api access

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, FromRequestParts, Json, Query, Request},
    http::request::Parts,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    auth::{hash_token, Token},
    endpoint::EndpointRejection,
    server::state::ServerState,
    types::ModelID,
};

pub mod db;
pub mod handlers;

/// Middleware for authenticating server api access
/// using `api_key` query param provided in the url
#[derive(Debug, Clone, Copy)]
pub struct ApiAuthentication;

#[async_trait]
impl FromRequestParts<ServerState> for ApiAuthentication {
    type Rejection = EndpointRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        let path = &parts.uri.path();

        if path.starts_with("/api/") {
            if crate::UNAUTHENTICATED_ENDPOINTS.contains(path) {
                return Ok(Self);
            }

            // Extract key from url query.
            let Ok(Query(key)) = Query::<ApiKeyQuery>::from_request_parts(parts, state).await
            else {
                tracing::debug!("Request rejected no api key found");
                return Err(EndpointRejection::unauthorized());
            };
            // Verify api key.
            if !ApiToken::exists(hash_token(key.api_key.as_bytes()), state.database()).await?
            {
                tracing::debug!("Request rejected invalid api key.");
                return Err(EndpointRejection::unauthorized());
            }
        }

        Ok(Self)
    }
}

// Helper struct for extracting a key from the url.
#[allow(missing_debug_implementations)]
#[derive(Clone, Deserialize)]
pub struct ApiKeyQuery {
    pub api_key: String,
}

// ===== MODELS impls =====

/// A `Vec` of Api tokens
pub type ApiTokenList = Vec<ApiToken>;

/// The model representing a row in the `regions` database table.
#[derive(Debug, Clone, Serialize)]
pub struct ApiToken {
    pub id: ModelID,
    pub user_id: Option<ModelID>,
    pub token: Vec<u8>,
    pub belongs_to: String,
    pub created_at: OffsetDateTime,
    pub last_used_at: OffsetDateTime,
    pub revoked: bool,
}

impl ApiToken {
    /// Creates a new Api key model from the database row
    #[must_use]
    pub fn from_row(
        id: ModelID,
        user_id: Option<ModelID>,
        token: Vec<u8>,
        belongs_to: String,
        created_at: OffsetDateTime,
        last_used_at: OffsetDateTime,
        revoked: bool,
    ) -> Self {
        Self {
            id,
            user_id,
            token,
            belongs_to,
            created_at,
            last_used_at,
            revoked,
        }
    }

    /// Generates a new Api key for this user.
    #[must_use]
    pub fn new_for_user(user_id: ModelID) -> (Self, String) {
        let Token { hash, plaintext } = Token::new_session();
        let api_key = Self {
            id: ModelID::new(),
            user_id: Some(user_id),
            token: hash.to_vec(),
            belongs_to: "USER_AUTH".to_owned(),
            created_at: OffsetDateTime::now_utc(),
            last_used_at: OffsetDateTime::now_utc(),
            revoked: false,
        };
        (api_key, plaintext)
    }

    /// Generates a new Api key for apps.
    #[must_use]
    pub fn new_for_app(belongs_to: String) -> (Self, String) {
        let Token { hash, plaintext } = Token::new_session();
        let api_key = Self {
            id: ModelID::new(),
            user_id: None,
            token: hash.to_vec(),
            belongs_to,
            created_at: OffsetDateTime::now_utc(),
            last_used_at: OffsetDateTime::now_utc(),
            revoked: false,
        };
        (api_key, plaintext)
    }
}

// ==== fORM impls =====

/// App api key create form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiTokenForAppForm {
    pub name: String,
}

#[async_trait]
impl FromRequest<ServerState> for ApiTokenForAppForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut token) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        token.name = token.name.trim().to_owned();

        Ok(token)
    }
}
