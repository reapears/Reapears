//! User profile forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json, Request},
};
use serde::Deserialize;

use crate::{
    endpoint::{
        validators::{TransformString, ValidateString},
        EndpointRejection, EndpointResult,
    },
    server::state::ServerState,
};

/// User profile update form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileUpdateForm {
    pub about: Option<String>,
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

impl UserProfileUpdateForm {
    /// Validates profile form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        if let Some(ref about) = self.about {
            about.validate_len(0, 512, "About must be at most  512 characters")?;
        }

        if let Some(ref lives_at) = self.lives_at {
            lives_at.validate_len(0, 128, "Lives at must be at most  128 characters")?;
        }

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.about = self.about.as_ref().map(|about| about.clean());
        self.lives_at = self.lives_at.as_ref().map(|lives_at| lives_at.clean());
    }
}

#[async_trait]
impl FromRequest<ServerState> for UserProfileUpdateForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut profile) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        profile.validate()?;

        Ok(profile)
    }
}
