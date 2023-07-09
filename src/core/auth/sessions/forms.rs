//! Session forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json},
    http::Request,
};
use serde::Deserialize;
use time::OffsetDateTime;
use validator::Validate;

use crate::{
    accounts::user::models::User,
    auth::{verify_password, Token, TokenHash},
    endpoint::EndpointRejection,
    server::state::ServerState,
    types::ModelID,
};

use super::models::Session;

/// User login form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginForm {
    #[validate(length(min = 1, max = 255, message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 1, max = 64, message = "Password too long"))]
    pub password: String,

    /// User id is set if user login completed successfully
    #[serde(skip_deserializing)]
    pub user_id: Option<ModelID>,
}

impl LoginForm {
    /// Creates new `SessionInsert` data and returns (`SessionInsert`, token:String)
    ///
    /// # Panics
    ///
    /// Panics if `user_id` is not set
    #[must_use]
    pub fn session_data(self, user_agent: String) -> (SessionInsert, String) {
        let token = Token::new_session();
        // Store the token hash at the server and return the plaintext to the user
        let (plaintext, token_hash) = token.into_parts();
        (
            SessionInsert {
                id: ModelID::new(),
                user_id: self.user_id.unwrap(),
                user_agent,
                token: token_hash,
                created_at: OffsetDateTime::now_utc(),
                last_used_at: OffsetDateTime::now_utc(),
            },
            plaintext,
        )
    }

    /// Sets `user_id`
    #[allow(clippy::missing_const_for_fn)]
    fn set_user_id(self, id: ModelID) -> Self {
        let mut this = self;
        this.user_id = Some(id);
        this
    }
}

/// Session cleaned data
#[derive(Debug, Clone)]
pub struct SessionInsert {
    pub id: ModelID,
    pub user_id: ModelID,
    pub user_agent: String,
    pub token: TokenHash,
    pub created_at: OffsetDateTime,
    pub last_used_at: OffsetDateTime,
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for LoginForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        const INVALID_CREDENTIALS: &str = "The username or password you have entered is invalid.";
        let Json(input) = Json::<Self>::from_request(req, state).await?;

        let db = state.database.clone();
        let email = input.email.clone();

        let Some(user) = Session::find_user_by_email(email.clone(), db.clone()).await? else{
            return Err(EndpointRejection::BadRequest(INVALID_CREDENTIALS.into()));
        };

        if !user.email_verified {
            tracing::info!("Login error, email not verified.");
            // Delete user account is not verified they must restart signup process
            User::delete_unverified(user.id, db).await?;
            return Err(EndpointRejection::BadRequest(
                "Sorry!, we could not find your account.".into(),
            ));
        }

        if user.account_locked {
            tracing::info!("Login error, account locked.");
            return Err(EndpointRejection::BadRequest(
                "Your account has been locked".into(),
            ));
        }

        // Authenticate the user; check the password in valid.
        if verify_password(&input.password, user.phc_string).await? {
            // NB! don't forget the set the user id
            Ok(input.set_user_id(user.id))
        } else {
            tracing::info!("Login error, password incorrect.");
            Err(EndpointRejection::BadRequest(INVALID_CREDENTIALS.into()))
        }
    }
}

/// Session update cleaned data
#[derive(Debug, Clone)]
pub struct SessionUpdate {
    pub last_used_at: OffsetDateTime,
}

impl SessionUpdate {
    /// Create new `SessionUpdate`
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            last_used_at: OffsetDateTime::now_utc(),
        }
    }
}

/// Login redirect after successful login
#[derive(Clone, Debug, Deserialize)]
pub struct SuccessRedirect {
    pub return_to: String,
}

impl Default for SuccessRedirect {
    fn default() -> Self {
        Self {
            return_to: String::from("/harvests"),
        }
    }
}
