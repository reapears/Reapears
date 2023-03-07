//! Model id impls

use axum::{
    async_trait,
    extract::{path, rejection::PathRejection, FromRequestParts},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use crate::endpoint::EndpointRejection;

/// Deserializes model id from request url
#[derive(Debug, Clone, Copy)]
pub struct ModelId<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for ModelId<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = EndpointRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match path::Path::<T>::from_request_parts(parts, state).await {
            Ok(path) => Ok(Self(path.0)),
            Err(rejection) => match rejection {
                PathRejection::FailedToDeserializePathParams(kind) => {
                    tracing::warn!("ModelId not found or invalid: {}", kind);
                    Err(EndpointRejection::not_found("page"))
                }
                PathRejection::MissingPathParams(err) => {
                    tracing::error!("ModelId extraction error,: {}.", err);
                    Err(EndpointRejection::internal_server_error())
                }
                other_errors => {
                    tracing::error!("ModelId extraction error, {}", other_errors);
                    Err(EndpointRejection::internal_server_error())
                }
            },
        }
    }
}
