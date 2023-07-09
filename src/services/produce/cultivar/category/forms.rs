//! Cultivar category forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json},
    http::Request,
};
use serde::Deserialize;
use validator::Validate;

use crate::{endpoint::EndpointRejection, server::state::ServerState, types::ModelID};

/// Cultivar category create form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CultivarCategoryForm {
    #[validate(length(min = 1, max = 32))]
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

#[async_trait]
impl<B> FromRequest<ServerState, B> for CultivarCategoryForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        let Json(input) = Json::<Self>::from_request(req, state).await?;
        match input.validate() {
            Ok(()) => Ok(input),
            Err(err) => {
                tracing::error!("Validation error: {}", err);
                Err(EndpointRejection::BadRequest(err.to_string().into()))
            }
        }
    }
}

// ===== CultivarCategory Update form impl =====

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
