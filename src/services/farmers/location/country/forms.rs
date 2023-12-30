//! Location country forms impls

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

/// Country create form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryForm {
    pub name: String,
}

/// Country create form cleaned data
#[derive(Debug, Clone)]
pub struct CountryInsertData {
    pub id: ModelID,
    pub name: String,
}

impl From<CountryForm> for CountryInsertData {
    fn from(form: CountryForm) -> Self {
        Self {
            id: ModelID::new(),
            name: form.name,
        }
    }
}

/// Country update form cleaned data
#[derive(Debug, Clone)]
pub struct CountryUpdateData {
    pub name: String,
}

impl From<CountryForm> for CountryUpdateData {
    fn from(form: CountryForm) -> Self {
        Self { name: form.name }
    }
}

impl CountryForm {
    /// Validates region form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.name
            .validate_len(0, 32, "Country name must be at most 32 characters")?;

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.name = self.name.clean().to_titlecase();
    }
}

#[async_trait]
impl FromRequest<ServerState> for CountryForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut country) = Json::<Self>::from_request(req, state).await?;

        // Validate from fields
        country.validate()?;

        Ok(country)
    }
}
