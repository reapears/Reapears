//! Require authorization mixin impl

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::PrivateCookieJar;
use uuid::Uuid;

use crate::{
    auth::{
        sessions::{forms::SessionUpdate, get_session_token_hash, models::Session},
        TokenHash,
    },
    endpoint::EndpointRejection,
    error::ServerResult,
    server::state::{DatabaseConnection, ServerState},
};

/// Authenticates user requests
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: Uuid,
    pub is_farmer: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
}

#[async_trait]
impl FromRequestParts<ServerState> for CurrentUser {
    type Rejection = EndpointRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        let db = state.database.clone();
        let key = state.cookie_key.clone();
        let cookie_jar = PrivateCookieJar::from_headers(&parts.headers, key);

        let Some(session_token) = get_session_token_hash(&cookie_jar) else{
            return Err(EndpointRejection::unauthorized());
        };

        let Some(current_user) = get_current_user(session_token, db.clone()).await? else{
            return Err(EndpointRejection::unauthorized());
        };

        // Update session last_used_at
        tokio::spawn(async move { Session::update(session_token, SessionUpdate::new(), db).await });

        Ok(current_user)
    }
}

impl CurrentUser {
    /// Creates a new `CurrentUser` from the database row
    const fn from_row(id: Uuid, is_farmer: bool, is_staff: bool, is_superuser: bool) -> Self {
        Self {
            id,
            is_farmer,
            is_staff,
            is_superuser,
        }
    }
}

/// Gets the user associated with the token
///
/// # Errors
///
/// Return database error
pub async fn get_current_user(
    token: TokenHash,
    db: DatabaseConnection,
) -> ServerResult<Option<CurrentUser>> {
    match sqlx::query!(
        r#"
            SELECT user_.id AS user_id,
                user_.is_farmer,
                user_.is_staff,
                user_.is_superuser
            FROM auth.sessions sessions
            LEFT JOIN accounts.users user_ 
                ON sessions.user_id = user_.id

            WHERE sessions.token = $1;
        "#,
        &token
    )
    .fetch_optional(&db.pool)
    .await
    {
        Ok(rec) => Ok(rec.map(|rec| {
            CurrentUser::from_row(rec.user_id, rec.is_farmer, rec.is_staff, rec.is_superuser)
        })),
        Err(err) => {
            tracing::error!("Database error, failed to fetch current-user: {}", err);
            Err(err.into())
        }
    }
}
