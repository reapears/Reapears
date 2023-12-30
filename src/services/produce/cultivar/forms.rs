//! Cultivar forms impls

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

/// Cultivar create form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CultivarCreateForm {
    pub name: String,
    pub category_id: String,
}

/// Cultivar create form cleaned data
#[derive(Debug, Clone)]
pub struct CultivarInsertData {
    pub id: ModelID,
    pub name: String,
    pub category_id: ModelID,
}

impl From<CultivarCreateForm> for CultivarInsertData {
    fn from(form: CultivarCreateForm) -> Self {
        Self {
            id: ModelID::new(),
            name: form.name,
            category_id: ModelID::from_str_unchecked(&form.category_id),
        }
    }
}

impl CultivarCreateForm {
    /// Validates cultivar form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.category_id.validate_id("Invalid category id")?;
        self.name
            .validate_len(0, 32, "Cultivar name must be at most 32 characters")?;

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.name = self.name.clean().to_titlecase();
    }
}

#[async_trait]
impl FromRequest<ServerState> for CultivarCreateForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut cultivar) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        cultivar.validate()?;

        Ok(cultivar)
    }
}

// ===== Cultivar UpdateForm impls ======

/// Cultivar update form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CultivarUpdateForm {
    pub name: String,
    pub category_id: String,
}

/// Cultivar update form cleaned data
#[derive(Debug, Clone)]
pub struct CultivarUpdateData {
    pub name: String,
    pub category_id: ModelID,
}

impl From<CultivarUpdateForm> for CultivarUpdateData {
    fn from(form: CultivarUpdateForm) -> Self {
        Self {
            category_id: ModelID::from_str_unchecked(form.category_id),
            name: form.name,
        }
    }
}

impl CultivarUpdateForm {
    /// Validates cultivar form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.category_id.validate_id("Invalid category id")?;
        self.name
            .validate_len(0, 32, "Cultivar name must be at most 32 characters")?;

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.name = self.name.clean().to_titlecase();
    }
}

#[async_trait]
impl FromRequest<ServerState> for CultivarUpdateForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut cultivar) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        cultivar.validate()?;

        Ok(cultivar)
    }
}
