// //! Email forms impls

use axum::async_trait;
use serde::Deserialize;
use time::OffsetDateTime;
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::{Token, TokenHash},
    db,
    endpoint::{EndpointRejection, EndpointResult, ModelId, ValidateForm},
    server::state::ServerState,
};

/// Email create form
#[derive(Clone, Debug, Deserialize, Validate)]
pub struct EmailForm {
    #[validate(email)]
    pub email: String,
}

impl EmailForm {
    // Return `EmailInsertPendingData` and verification code
    #[must_use]
    pub fn pending_update_data(self) -> (EmailInsertPendingData, String) {
        let (plaintext, hash) = Token::new_code().into_parts();
        (EmailInsertPendingData::new(self.email, hash), plaintext)
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
impl ValidateForm<ServerState> for EmailForm {
    #[tracing::instrument(skip(self, _state), name = "Validate EmailForm")]
    async fn validate_form(
        self,
        _state: &ServerState,
        _model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        match self.validate() {
            Ok(()) => Ok(self),
            Err(err) => Err(EndpointRejection::BadRequest(err.to_string().into())),
        }
    }
}

// ---PendingUpdate---

/// Email pending update cleaned data
#[derive(Debug, Clone)]
pub struct EmailInsertPendingData {
    pub id: Uuid,
    pub new_email: String,
    pub token: TokenHash,
    pub token_generated_at: OffsetDateTime,
}

impl EmailInsertPendingData {
    /// Creates new email pending update insert data
    fn new(new_email: String, token: TokenHash) -> Self {
        Self {
            id: db::model_id(),
            new_email,
            token,
            token_generated_at: OffsetDateTime::now_utc(),
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
impl ValidateForm<ServerState> for CodeConfirmForm {
    #[tracing::instrument(skip(self, _state), name = "Validate CodeConfirmForm")]
    async fn validate_form(
        self,
        _state: &ServerState,
        _model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        match self.validate() {
            Ok(()) => Ok(self),
            Err(err) => Err(EndpointRejection::BadRequest(err.to_string().into())),
        }
    }
}
