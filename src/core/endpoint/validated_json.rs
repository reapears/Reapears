//! Json deserializer that validates user input impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest},
    http::Request,
    Json, RequestExt,
};
use serde::de::DeserializeOwned;
use uuid::Uuid;

use crate::endpoint::{EndpointRejection, EndpointResult, ModelId};

/// Trait for validating user input forms
#[async_trait]
pub trait ValidateForm<S>
where
    S: Send + Sync,
    Self: Sized + Send,
{
    /// Validates user inputs
    async fn validate_form(
        self,
        state: &S,
        model_id: Option<ModelId<Uuid>>,
        // user: Option<CurrentUser>,
    ) -> EndpointResult<Self>;
}

/// A validated Json extractor
///
/// it validates body by calling `validate_form`
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
    T: DeserializeOwned + ValidateForm<S>,
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    // these bounds are required by `async_trait`
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = EndpointRejection;

    async fn from_request(mut req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let model_id = req.extract_parts::<ModelId<Uuid>>().await.ok();
        match Json::<T>::from_request(req, state).await {
            Ok(Json(input)) => match input.validate_form(state, model_id).await {
                Ok(validated_input) => Ok(Self(validated_input)),
                Err(err) => Err(err),
            },
            Err(err) => Err(err.into()),
        }
    }
}
