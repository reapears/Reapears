//! Location country forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json},
    http::Request,
};
use serde::Deserialize;

use validator::Validate;

use crate::{endpoint::EndpointRejection, server::state::ServerState, types::ModelID};

/// Country create form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CountryForm {
    #[validate(length(min = 1, max = 32))]
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

#[async_trait]
impl<B> FromRequest<ServerState, B> for CountryForm
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

// ===== Country update impl =====

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
