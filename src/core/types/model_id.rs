//! Model id impls

use std::fmt;

use axum::{
    async_trait,
    extract::{path, rejection::PathRejection, FromRequestParts},
    http::request::Parts,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{endpoint::EndpointRejection, error::ServerError};

/// An Id used to uniquely identify a model
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct ModelID(pub Uuid);

impl Default for ModelID {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelID {
    /// Generates a new Id
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    /// Try parsing `ModelID` from a String
    ///  
    /// # Panics
    ///
    /// Panics if a string is not a valid Uuid
    pub fn from_str_unchecked<S>(id: S) -> Self
    where
        S: AsRef<str>,
    {
        Self(uuid::Uuid::parse_str(id.as_ref()).unwrap())
    }
}

impl TryFrom<&str> for ModelID {
    type Error = ServerError;
    fn try_from(id: &str) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(id).map_err(|err| {
            ServerError::new(format!(
                "Failed to parse ModelID from string `{id}`: {err}."
            ))
        })?;

        Ok(Self(id))
    }
}

impl fmt::Display for ModelID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ModelID {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl std::cmp::PartialEq<Uuid> for ModelID {
    fn eq(&self, other: &Uuid) -> bool {
        self.0 == *other
    }
}

impl std::cmp::PartialEq<ModelID> for Uuid {
    fn eq(&self, other: &ModelID) -> bool {
        *self == other.0
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for ModelID
where
    S: Send + Sync,
{
    type Rejection = EndpointRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match path::Path::<Uuid>::from_request_parts(parts, state).await {
            Ok(path) => Ok(Self(path.0)),
            Err(rejection) => match rejection {
                PathRejection::FailedToDeserializePathParams(kind) => {
                    tracing::warn!("Model Id not found or invalid: {}", kind);
                    Err(EndpointRejection::not_found("page"))
                }
                PathRejection::MissingPathParams(err) => {
                    tracing::error!("Model Id extraction error,: {}.", err);
                    Err(EndpointRejection::internal_server_error())
                }
                other_errors => {
                    tracing::error!("Model Id extraction error, {}", other_errors);
                    Err(EndpointRejection::internal_server_error())
                }
            },
        }
    }
}
