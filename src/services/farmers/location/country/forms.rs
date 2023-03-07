//! Location country forms impls

use axum::async_trait;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db,
    endpoint::{EndpointRejection, EndpointResult, ModelId, ValidateForm},
    server::state::ServerState,
};

/// Country create form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CountryForm {
    #[validate(length(min = 1, max = 32))]
    pub name: String,
}

/// Country create form cleaned data
#[derive(Debug, Clone)]
pub struct CountryInsertData {
    pub id: Uuid,
    pub name: String,
}

impl From<CountryForm> for CountryInsertData {
    fn from(form: CountryForm) -> Self {
        Self {
            id: db::model_id(),
            name: form.name,
        }
    }
}

#[async_trait]
impl ValidateForm<ServerState> for CountryForm {
    #[tracing::instrument(skip(self, _state), name = "Validate CountryForm")]
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

// === Country update impl ===

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
