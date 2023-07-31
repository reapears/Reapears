//! Require authorization mixin impl

use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use axum_extra::extract::PrivateCookieJar;

use crate::{
    auth::{
        self,
        api_key::ApiKeyQuery,
        sessions::{forms::SessionUpdate, get_session_token_hash, models::Session},
        TokenHash,
    },
    endpoint::{EndpointRejection, EndpointResult},
    error::ServerResult,
    server::state::{DatabaseConnection, ServerState},
    types::ModelID,
};

/// Authenticates user requests
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: ModelID,
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

        let user = match get_session_token_hash(&cookie_jar) {
            Some(session_token) => Self::from_cookies(session_token, db.clone()).await?,
            None => Self::from_api_key(parts, state).await?,
        };

        // Cache current user
        parts.extensions.insert(user.clone());

        Ok(user)
    }
}

impl CurrentUser {
    /// Returns the id of the user
    #[must_use]
    pub fn id(&self) -> ModelID {
        self.id
    }

    /// Creates a new `CurrentUser` from the database row
    const fn from_row(id: ModelID, is_farmer: bool, is_staff: bool, is_superuser: bool) -> Self {
        Self {
            id,
            is_farmer,
            is_staff,
            is_superuser,
        }
    }

    /// Extract cached `CurrentUser` from `Extensions`
    pub async fn from_parts(parts: &mut Parts, state: &ServerState) -> EndpointResult<Self> {
        match parts.extensions.get::<Self>() {
            Some(user) => Ok(user.clone()),
            None => Self::from_request_parts(parts, state).await,
        }
    }

    /// Authenticate and get user by Session token.
    pub async fn from_cookies(token: TokenHash, db: DatabaseConnection) -> EndpointResult<Self> {
        let Some(user) = get_current_user(token, db.clone()).await? else{
            return Err(EndpointRejection::unauthorized());
        };

        // Update session last_used_at
        tokio::spawn(async move { Session::update(token, SessionUpdate::new(), db).await });

        Ok(user)
    }

    /// Authenticate and get user by Api key.
    pub async fn from_api_key(parts: &mut Parts, state: &ServerState) -> EndpointResult<Self> {
        let Ok(Query(key)) = Query::<ApiKeyQuery>::from_request_parts(parts, state).await else{
            tracing::debug!("Request rejected no api key found");
            return Err(EndpointRejection::unauthorized());
        };

        let token = auth::hash_token(key.api_key.as_bytes());
        let Some(user) = get_current_user_by_api_key(token, state.database.clone()).await? else{
            return Err(EndpointRejection::unauthorized());
        };

        Ok(user)
    }
}

// ===== SuperUser =====

/// Authenticated user who has the most privileged access
#[derive(Debug, Clone)]
pub struct SuperUser(pub CurrentUser);

#[async_trait]
impl FromRequestParts<ServerState> for SuperUser {
    type Rejection = EndpointRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        let user = CurrentUser::from_request_parts(parts, state).await?;
        if !user.is_superuser {
            tracing::trace!("Request rejected superuser privilege required.");
            return Err(EndpointRejection::forbidden());
        }
        Ok(Self(user))
    }
}

impl SuperUser {
    /// Returns the id of the user
    #[must_use]
    pub const fn id(&self) -> ModelID {
        self.0.id
    }

    /// Extract cached `SuperUser` from `Extensions`
    pub async fn from_parts(parts: &mut Parts, state: &ServerState) -> EndpointResult<Self> {
        let user = CurrentUser::from_parts(parts, state).await?;
        if !user.is_superuser {
            return Err(EndpointRejection::forbidden());
        }
        Ok(Self(user))
    }
}

// ===== Admin User =====

/// Privileged authenticated user
/// User that can access the admin page
#[derive(Debug, Clone)]
pub struct AdminUser(pub CurrentUser);

#[async_trait]
impl FromRequestParts<ServerState> for AdminUser {
    type Rejection = EndpointRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        let user = CurrentUser::from_request_parts(parts, state).await?;
        if !user.is_staff {
            tracing::trace!("Request rejected is staff privilege required.");
            return Err(EndpointRejection::forbidden());
        }
        Ok(Self(user))
    }
}

impl AdminUser {
    /// Returns whether the user has the out out privilege
    #[must_use]
    pub const fn is_superuser(&self) -> bool {
        self.0.is_superuser
    }

    /// Returns the id of the user
    #[must_use]
    pub const fn id(&self) -> ModelID {
        self.0.id
    }

    /// Extract cached `AdminUser` from `Extensions`
    pub async fn from_parts(parts: &mut Parts, state: &ServerState) -> EndpointResult<Self> {
        let user = CurrentUser::from_parts(parts, state).await?;
        if !user.is_staff {
            tracing::trace!("Request rejected staff privilege required.");
            return Err(EndpointRejection::forbidden());
        }
        Ok(Self(user))
    }
}

// ===== Farmer User =====

/// Authenticated user who is a farmer
#[derive(Debug, Clone)]
pub struct FarmerUser(pub CurrentUser);

#[async_trait]
impl FromRequestParts<ServerState> for FarmerUser {
    type Rejection = EndpointRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        let user = CurrentUser::from_request_parts(parts, state).await?;
        if !user.is_farmer {
            return Err(EndpointRejection::Forbidden(
                "Create a farm first to perform this action".into(),
            ));
        }
        Ok(Self(user))
    }
}

impl FarmerUser {
    /// Returns the id of the user
    #[must_use]
    pub const fn id(&self) -> ModelID {
        self.0.id
    }

    /// Extract cached `FarmerUser` from `Extensions`
    pub async fn from_parts(parts: &mut Parts, state: &ServerState) -> EndpointResult<Self> {
        let user = CurrentUser::from_parts(parts, state).await?;
        if !user.is_farmer {
            return Err(EndpointRejection::Forbidden(
                "Create a farm first to perform this action".into(),
            ));
        }
        Ok(Self(user))
    }
}

/// Gets the user associated with the session token
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
            CurrentUser::from_row(
                rec.user_id.into(),
                rec.is_farmer,
                rec.is_staff,
                rec.is_superuser,
            )
        })),
        Err(err) => {
            tracing::error!("Database error, failed to fetch current-user: {}", err);
            Err(err.into())
        }
    }
}

/// Gets the user associated with the api key
///
/// # Errors
///
/// Return database error
pub async fn get_current_user_by_api_key(
    token: TokenHash,
    db: DatabaseConnection,
) -> ServerResult<Option<CurrentUser>> {
    match sqlx::query!(
        r#"
            SELECT user_.id AS user_id,
                user_.is_farmer,
                user_.is_staff,
                user_.is_superuser
            FROM auth.api_tokens token
            LEFT JOIN accounts.users user_ 
                ON token.user_id = user_.id

            WHERE token.token = $1;
        "#,
        &token
    )
    .fetch_optional(&db.pool)
    .await
    {
        Ok(rec) => Ok(rec.map(|rec| {
            CurrentUser::from_row(
                rec.user_id.into(),
                rec.is_farmer,
                rec.is_staff,
                rec.is_superuser,
            )
        })),
        Err(err) => {
            tracing::error!("Database error, failed to fetch current-user: {}", err);
            Err(err.into())
        }
    }
}
