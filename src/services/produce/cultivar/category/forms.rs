//! Cultivar category forms impls

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

/// Cultivar category create form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CultivarCategoryForm {
    pub name: String,
}

/// Cultivar category create form cleaned data
#[derive(Debug, Clone)]
pub struct CultivarCategoryInsertData {
    pub id: ModelID,
    pub name: String,
}

impl From<CultivarCategoryForm> for CultivarCategoryInsertData {
    fn from(form: CultivarCategoryForm) -> Self {
        Self {
            id: ModelID::new(),
            name: form.name,
        }
    }
}

/// Cultivar category update form cleaned data
#[derive(Debug, Clone)]
pub struct CultivarCategoryUpdateData {
    pub name: String,
}

impl From<CultivarCategoryForm> for CultivarCategoryUpdateData {
    fn from(form: CultivarCategoryForm) -> Self {
        Self { name: form.name }
    }
}

impl CultivarCategoryForm {
    /// Validates cultivar category form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.name
            .validate_len(0, 32, "Cultivar category must be at most 32 characters")?;

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.name = self.name.clean().to_titlecase();
    }
}

#[async_trait]
impl FromRequest<ServerState> for CultivarCategoryForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut category) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        category.validate()?;

        Ok(category)
    }
}
