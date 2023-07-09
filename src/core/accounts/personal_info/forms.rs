//! User personal info forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json},
    http::Request,
};
use serde::Deserialize;
use time::Date;
use validator::Validate;

use crate::{endpoint::EndpointRejection, server::state::ServerState};

/// User personal info update form
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PersonalInfoUpdateForm {
    #[validate(length(min = 1, max = 16))]
    pub first_name: Option<String>,

    #[validate(length(min = 1, max = 16))]
    pub last_name: Option<String>,

    #[validate(length(min = 1, max = 6, message = "Invalid gender"))]
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

#[async_trait]
impl<B> FromRequest<ServerState, B> for PersonalInfoUpdateForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        let Json(input) = Json::<Self>::from_request(req, state).await?;

        match input.validate() {
            Ok(()) => {
                if let Some(ref gender) = input.gender {
                    helpers::validate_gender(gender)?;
                }
                Ok(input)
            }
            Err(err) => Err(EndpointRejection::BadRequest(err.to_string().into())),
        }
    }
}

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
