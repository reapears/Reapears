// //! Email forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json},
    http::Request,
};
use serde::Deserialize;
use time::OffsetDateTime;
use validator::Validate;

use crate::{
    auth::{Token, TokenHash},
    endpoint::EndpointRejection,
    server::state::ServerState,
    types::ModelID,
};

/// Email create form
#[derive(Clone, Debug, Deserialize, Validate)]
pub struct EmailForm {
    #[validate(email)]
    pub email: String,
}

impl EmailForm {
    // Return (`EmailInsertPendingData`, approve_plaintext, verify_plaintext)
    #[must_use]
    pub fn pending_update_data(self) -> (EmailInsertPendingData, String) {
        let (approve_text, approve_hash) = Token::new_code().into_parts();
        let values = EmailInsertPendingData::new(self.email, approve_hash);
        (values, approve_text)
    }
}

/// Email insert cleaned data
#[derive(Debug, Clone)]
pub struct EmailInsertData {
    pub email: String,
    pub verified: bool,
    pub token: TokenHash,
    pub token_generated_at: OffsetDateTime,
}

impl EmailInsertData {
    #[must_use]
    pub fn new(email: String, token: TokenHash) -> Self {
        Self {
            email,
            verified: false,
            token,
            token_generated_at: OffsetDateTime::now_utc(),
        }
    }
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for EmailForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        let Json(input) = Json::<Self>::from_request(req, state).await?;

        match input.validate() {
            Ok(()) => Ok(input),
            Err(err) => Err(EndpointRejection::BadRequest(err.to_string().into())),
        }
    }
}

// ===== Email Pending Updates impls =====

/// Email pending update cleaned data
#[derive(Debug, Clone)]
pub struct EmailInsertPendingData {
    pub id: ModelID,
    pub new_email: String,
    pub previous_email_approval_code: TokenHash,
    pub email_change_approved: bool,
    pub generated_at: OffsetDateTime,
}

impl EmailInsertPendingData {
    /// Creates new email pending update insert data
    fn new(new_email: String, approval_token: TokenHash) -> Self {
        Self {
            id: ModelID::new(),
            new_email,
            previous_email_approval_code: approval_token,
            email_change_approved: false,
            generated_at: OffsetDateTime::now_utc(),
        }
    }
}

// ---CodeConfirm---

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CodeConfirmForm {
    #[validate(length(equal = 6))] // the length is 6
    pub code: String,
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for CodeConfirmForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        let Json(input) = Json::<Self>::from_request(req, state).await?;
        match input.validate() {
            Ok(()) => Ok(input),
            Err(err) => Err(EndpointRejection::BadRequest(err.to_string().into())),
        }
    }
}
