//! Password forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json},
    http::Request,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    auth::hash_password, endpoint::EndpointRejection, error::ServerResult,
    server::state::ServerState,
};

/// User password verify form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct PasswordVerifyForm {
    pub password: String,
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for PasswordVerifyForm
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
impl<B> FromRequest<ServerState, B> for PasswordChangeForm
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

// ---ForgotForm---

// USer password forgot form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct PasswordForgotForm {
    pub email: String,
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for PasswordForgotForm
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
impl<B> FromRequest<ServerState, B> for PasswordResetForm
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
