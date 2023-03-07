//! Cultivar category forms impls

use axum::async_trait;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db,
    endpoint::{EndpointRejection, EndpointResult, ModelId, ValidateForm},
    server::state::ServerState,
};

// TODO! validate

/// Cultivar category create form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CultivarCategoryForm {
    #[validate(length(min = 1, max = 32))]
    pub name: String,
}

/// Cultivar category create form cleaned data
#[derive(Debug, Clone)]
pub struct CultivarCategoryInsertData {
    pub id: Uuid,
    pub name: String,
}

impl From<CultivarCategoryForm> for CultivarCategoryInsertData {
    fn from(form: CultivarCategoryForm) -> Self {
        Self {
            id: db::model_id(),
            name: form.name,
        }
    }
}

#[async_trait]
impl ValidateForm<ServerState> for CultivarCategoryForm {
    #[tracing::instrument(skip(self, _state), name = "Validate CultivarCategoryCreateForm")]
    async fn validate_form(
        self,
        _state: &ServerState,
        _model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        match self.validate() {
            Ok(()) => Ok(self),
            Err(err) => {
                tracing::error!("Validation error: {}", err);
                Err(EndpointRejection::BadRequest(err.to_string().into()))
            }
        }
    }
}

// === CultivarCategory update impl ===

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
