//! User personal info forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json, Request},
};
use serde::Deserialize;
use time::Date;

use crate::{
    endpoint::{
        validators::{TransformString, ValidateString},
        EndpointRejection, EndpointResult,
    },
    server::state::ServerState,
};

/// User personal info update form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalInfoUpdateForm {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<Date>,
}

/// User personal info cleaned data
#[derive(Debug, Clone)]
pub struct PersonalInfoUpdateData {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<Date>,
}

impl From<PersonalInfoUpdateForm> for PersonalInfoUpdateData {
    fn from(form: PersonalInfoUpdateForm) -> Self {
        Self {
            first_name: form.first_name,
            last_name: form.last_name,
            gender: form.gender,
            date_of_birth: form.date_of_birth,
        }
    }
}

impl PersonalInfoUpdateForm {
    /// Validates personal info form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        if let Some(ref first_name) = self.first_name {
            first_name.validate_len(0, 24, "first name must be at most 24 characters")?;
        }

        if let Some(ref last_name) = self.last_name {
            last_name.validate_len(0, 24, "last name must be at most 24 characters")?;
        }

        if let Some(ref gender) = self.gender {
            helpers::validate_gender(gender)?;
        }

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.first_name = self
            .first_name
            .as_ref()
            .map(|first_name| first_name.clean());
        self.last_name = self.last_name.as_ref().map(|last_name| last_name.clean());
        self.gender = self.gender.as_ref().map(|gender| gender.clean());
    }
}

#[async_trait]
impl FromRequest<ServerState> for PersonalInfoUpdateForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut personal_info) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        personal_info.validate()?;

        Ok(personal_info)
    }
}

// ===== Helpers impls =====

mod helpers {
    use crate::endpoint::{EndpointRejection, EndpointResult};

    /// Validate user gender
    pub fn validate_gender(gender: &str) -> EndpointResult<()> {
        let gender = gender.to_lowercase();
        if ["male", "female", "other"].contains(&gender.as_str()) {
            Ok(())
        } else {
            Err(EndpointRejection::BadRequest("Invalid gender".into()))
        }
    }
}
