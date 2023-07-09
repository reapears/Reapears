//! User profile forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json},
    http::Request,
};
use serde::Deserialize;

use validator::Validate;

use crate::{endpoint::EndpointRejection, server::state::ServerState};

/// User profile update form
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileUpdateForm {
    #[validate(length(min = 1, max = 512))]
    pub about: Option<String>,

    #[validate(length(min = 1, max = 128, message = "location too long"))]
    pub lives_at: Option<String>,
}

/// User profile update cleaned data
#[derive(Debug, Clone, Default)]
pub struct UserProfileUpdateData {
    pub about: String,
    pub lives_at: Option<String>,
}

impl From<UserProfileUpdateForm> for UserProfileUpdateData {
    fn from(form: UserProfileUpdateForm) -> Self {
        Self {
            about: form.about.unwrap_or_default(),
            lives_at: form.lives_at,
        }
    }
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for UserProfileUpdateForm
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
