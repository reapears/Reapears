//! Location region forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json},
    http::Request,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    endpoint::EndpointRejection, server::state::ServerState,
    services::farmers::location::forms::validate_country_id, types::ModelID,
};

/// Region create form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct RegionForm {
    #[validate(length(min = 1, max = 32))]
    pub name: String,

    #[validate(length(min = 1, max = 64, message = "Invalid country id"))]
    pub country_id: String,
}

/// Region create form cleaned data
#[derive(Debug, Clone)]
pub struct RegionInsertData {
    pub id: ModelID,
    pub country_id: ModelID,
    pub name: String,
}

impl From<RegionForm> for RegionInsertData {
    fn from(form: RegionForm) -> Self {
        Self {
            id: ModelID::new(),
            country_id: ModelID::from_str_unchecked(&form.country_id),
            name: form.name,
        }
    }
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for RegionForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        let Json(input) = Json::<Self>::from_request(req, state).await?;

        match input.validate() {
            Ok(()) => {
                // Validate country id exists
                let Ok(country_id) = ModelID::try_from(input.country_id.as_str()) else{
                    return Err(EndpointRejection::BadRequest("Country not found".into()));
                };
                validate_country_id(country_id, state.database.clone()).await?;

                Ok(input)
            }
            Err(err) => {
                tracing::error!("Validation error: {}", err);
                Err(EndpointRejection::BadRequest(err.to_string().into()))
            }
        }
    }
}

// ===== Region Update form impl =====

/// Region update form cleaned data
#[derive(Debug, Clone)]
pub struct RegionUpdateData {
    pub name: Option<String>,
    pub country_id: Option<ModelID>,
}

impl From<RegionForm> for RegionUpdateData {
    fn from(form: RegionForm) -> Self {
        Self {
            name: Some(form.name),
            country_id: Some(ModelID::from_str_unchecked(&form.country_id)),
        }
    }
}
