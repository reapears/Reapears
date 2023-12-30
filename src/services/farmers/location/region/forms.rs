//! Location region forms impls

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
    types::ModelID,
};

/// Region create form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionForm {
    pub name: String,
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

impl RegionForm {
    /// Validates region form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.name
            .validate_len(0, 32, "Country region name must be at most 32 characters")?;

        self.country_id.validate_id("Invalid country id")?;

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.name = self.name.clean().to_titlecase();
    }
}

#[async_trait]
impl FromRequest<ServerState> for RegionForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut region) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        region.validate()?;

        Ok(region)
    }
}
