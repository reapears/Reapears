//! Password forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json, Request},
};
use serde::Deserialize;

use crate::{
    auth::hash_password,
    endpoint::{
        validators::{TransformString, ValidateString},
        EndpointRejection, EndpointResult,
    },
    error::ServerResult,
    server::state::ServerState,
};

/// User password verify form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordVerifyForm {
    pub password: String,
}

impl PasswordVerifyForm {
    /// Validates password verify form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        self.password.validate_len(0, 25, "Incorrect password")?;

        Ok(())
    }

    // /// Clean form data
    // fn clean_data(&mut self) {
    //     self.password = self.password.clean();
    // }
}

#[async_trait]
impl FromRequest<ServerState> for PasswordVerifyForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut password_verify) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        password_verify.validate()?;

        Ok(password_verify)
    }
}

// ===== Password Change Form impls =====

/// User password change form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordChangeForm {
    pub current: String,
    pub new: String,
    pub confirm: String,
}

impl PasswordChangeForm {
    /// Validates password change form inputs
    fn validate(&self) -> EndpointResult<()> {
        self.current.validate_len(0, 25, "Incorrect password")?;

        self.new.validate_len(
            6,
            24,
            "password must be as least 6 character and at most 24 characters long",
        )?;

        if self.new != self.confirm {
            return Err(EndpointRejection::BadRequest(
                "new and confirm password must be the same.".into(),
            ));
        }

        Ok(())
    }

    /// Hash new password and return `phc_string`
    pub async fn try_phc(self) -> ServerResult<String> {
        hash_password(self.new).await
    }
}

#[async_trait]
impl FromRequest<ServerState> for PasswordChangeForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(password_change) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        password_change.validate()?;

        Ok(password_change)
    }
}

// ===== Password ForgotForm impls =====

// User password forgot form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordForgotForm {
    pub email: String,
}

impl PasswordForgotForm {
    /// Validates password forgot form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean form fields
        self.clean_data();

        self.email.validate_email()?;

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.email = self.email.clean().to_ascii_lowercase();
    }
}

#[async_trait]
impl FromRequest<ServerState> for PasswordForgotForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut password_forgot) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        password_forgot.validate()?;

        Ok(password_forgot)
    }
}

// ===== Password ResetForm impls =====

// User password reset form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordResetForm {
    pub new: String,
    pub confirm: String,
}

impl PasswordResetForm {
    /// Validates password reset form inputs
    fn validate(&self) -> EndpointResult<()> {
        self.new.validate_len(
            6,
            24,
            "password must be as least 6 character and at most 24 characters long",
        )?;

        if self.new != self.confirm {
            return Err(EndpointRejection::BadRequest(
                "new and confirm password must be the same.".into(),
            ));
        }

        Ok(())
    }

    /// Hash new password and return `phc_string`
    pub async fn try_phc(self) -> ServerResult<String> {
        hash_password(self.new).await
    }
}

#[async_trait]
impl FromRequest<ServerState> for PasswordResetForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        let Json(password_reset) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        password_reset.validate()?;

        Ok(password_reset)
    }
}
