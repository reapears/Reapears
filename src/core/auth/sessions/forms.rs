//! Session forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json, Request},
};
use serde::Deserialize;
use time::OffsetDateTime;

use crate::{
    accounts::{user::models::User, AccountDelete},
    auth::{verify_password, Token, TokenHash},
    endpoint::{
        validators::{TransformString, ValidateString},
        EndpointRejection, EndpointResult,
    },
    server::state::ServerState,
    types::ModelID,
};

use super::models::Session;

/// User login form
#[derive(Debug, Clone, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
    /// User id is set if user login completed successfully
    #[serde(skip_deserializing)]
    pub user_id: Option<ModelID>,
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

impl LoginForm {
    /// Validates email form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.email
            .validate_len(1, 255, crate::INVALID_CREDENTIALS_ERR_MSG)?;
        self.password
            .validate_len(0, 25, crate::INVALID_CREDENTIALS_ERR_MSG)?;

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.email = self.email.clean().to_ascii_lowercase();
    }

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

#[async_trait]
impl FromRequest<ServerState> for LoginForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        let Json(mut login) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        login.validate()?;

        let db = state.database();
        let email = login.email.clone();

        let Some(user) = Session::find_user_by_email(email.clone(), db.clone()).await? else {
            return Err(EndpointRejection::BadRequest(
                crate::INVALID_CREDENTIALS_ERR_MSG.into(),
            ));
        };

        if !user.email_verified {
            tracing::info!("Login error, email not verified.");
            // Delete user account is not verified they must restart signup process
            User::delete_unverified(user.id, db.clone()).await?;
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
        verify_password(&login.password, user.phc_string).await?;

        // Lift account delete request
        if user.requested_account_delete {
            AccountDelete::delete_request(user.id, db).await?;
        }

        // NB! don't forget the set the user id
        Ok(login.set_user_id(user.id))
    }
}

// ===== Session Update impls =====

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

// ===== Login RedirectTo impls =====

/// Login redirect after successful login
#[derive(Clone, Debug, Deserialize)]
pub struct SuccessRedirect {
    pub return_to: String,
}

impl Default for SuccessRedirect {
    fn default() -> Self {
        Self {
            return_to: String::from("/"),
        }
    }
}
