//! User profile forms impls

use axum::async_trait;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    endpoint::{EndpointRejection, EndpointResult, ModelId, ValidateForm},
    server::state::ServerState,
};

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
impl ValidateForm<ServerState> for UserProfileUpdateForm {
    #[tracing::instrument(skip(self, _state), name = "Validate UserProfileUpdateForm")]
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
