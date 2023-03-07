//! Location region forms impls

use axum::async_trait;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db,
    endpoint::{
        validators::{parse_uuid, unwrap_uuid},
        EndpointRejection, EndpointResult, ModelId, ValidateForm,
    },
    server::state::ServerState,
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
    pub id: Uuid,
    pub country_id: Uuid,
    pub name: String,
}

impl From<RegionForm> for RegionInsertData {
    fn from(form: RegionForm) -> Self {
        Self {
            id: db::model_id(),
            country_id: unwrap_uuid(&form.country_id),
            name: form.name,
        }
    }
}

#[async_trait]
impl ValidateForm<ServerState> for RegionForm {
    #[tracing::instrument(skip(self, _state), name = "Validate RegionForm")]
    async fn validate_form(
        self,
        _state: &ServerState,
        _model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        match self.validate() {
            Ok(()) => {
                parse_uuid(
                    &self.country_id,
                    "Location country not found",
                    "Invalid country id",
                )?;

                Ok(self)
            }
            Err(err) => {
                tracing::error!("Validation error: {}", err);
                Err(EndpointRejection::BadRequest(err.to_string().into()))
            }
        }
    }
}

// === Region update impl ===

/// Region update form cleaned data
#[derive(Debug, Clone)]
pub struct RegionUpdateData {
    pub name: Option<String>,
    pub country_id: Option<Uuid>,
}

impl From<RegionForm> for RegionUpdateData {
    fn from(form: RegionForm) -> Self {
        Self {
            name: Some(form.name),
            country_id: Some(unwrap_uuid(&form.country_id)),
        }
    }
}
