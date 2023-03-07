//! Password forms impls

use axum::async_trait;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::hash_password,
    endpoint::{EndpointRejection, EndpointResult, ModelId, ValidateForm},
    error::ServerResult,
    server::state::ServerState,
};

/// User password verify form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct PasswordVerifyForm {
    pub password: String,
}

#[async_trait]
impl ValidateForm<ServerState> for PasswordVerifyForm {
    #[tracing::instrument(skip(self, _state), name = "Validate PasswordVerifyForm")]
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

// ---ChangeForm---

/// User password change form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct PasswordChangeForm {
    pub current: String,
    pub new: String,
    pub confirm: String,
}

impl PasswordChangeForm {
    /// Hash new password and return `phc_string`
    pub async fn try_phc(self) -> ServerResult<String> {
        hash_password(self.new).await
    }
}

#[async_trait]
impl ValidateForm<ServerState> for PasswordChangeForm {
    #[tracing::instrument(skip(self, _state), name = "Validate PasswordChangeForm")]
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

// ---ForgotForm---

// USer password forgot form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct PasswordForgotForm {
    pub email: String,
}

#[async_trait]
impl ValidateForm<ServerState> for PasswordForgotForm {
    #[tracing::instrument(skip(self, _state), name = "Validate PasswordForgotForm")]
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

// ---ResetForm----

// User password reset form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct PasswordResetForm {
    pub new: String,
    pub confirm: String,
}

impl PasswordResetForm {
    /// Hash new password and return `phc_string`
    pub async fn try_phc(self) -> ServerResult<String> {
        hash_password(self.new).await
    }
}

#[async_trait]
impl ValidateForm<ServerState> for PasswordResetForm {
    #[tracing::instrument(skip(self, _state), name = "Validate PasswordResetForm")]
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
