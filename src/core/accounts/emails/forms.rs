// //! Email forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json, Request},
};
use serde::Deserialize;
use time::OffsetDateTime;

use crate::{
    auth::{Token, TokenHash},
    endpoint::{
        validators::{TransformString, ValidateString},
        EndpointRejection, EndpointResult,
    },
    server::state::ServerState,
    types::ModelID,
};

/// Email create form
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailForm {
    pub email: String,
}

/// Email insert cleaned data,
/// used for account registration.
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

/// Email pending update cleaned data
/// used for updating email.
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

impl EmailForm {
    /// Validates email form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.email.validate_email()?;

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.email = self.email.clean().to_ascii_lowercase();
    }

    // Return (`EmailInsertPendingData`, approve_plaintext)
    #[must_use]
    pub fn pending_update_data(self) -> (EmailInsertPendingData, String) {
        let (approve_text, approve_hash) = Token::new_code().into_parts();
        let values = EmailInsertPendingData::new(self.email, approve_hash);
        (values, approve_text)
    }
}

#[async_trait]
impl FromRequest<ServerState> for EmailForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut email) = Json::<Self>::from_request(req, state).await?;

        // Validate email form
        email.validate()?;

        Ok(email)
    }
}

// ===== Code ConfirmForm impls =====

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeConfirmForm {
    pub code: String,
}

impl CodeConfirmForm {
    /// Validates email form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.code.validate_len(6, 6, "Invalid code")?;

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.code = self.code.clean();
    }
}

#[async_trait]
impl FromRequest<ServerState> for CodeConfirmForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut code) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        code.validate()?;

        Ok(code)
    }
}
