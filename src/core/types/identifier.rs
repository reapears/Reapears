//! Model identifier impls

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A `Vec` of identifiers
pub type ModelIndex = Vec<ModelIdentifier>;

/// Unique identifier for models
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ModelIdentifier {
    pub id: Option<Uuid>,
    pub name: Option<String>,
}

impl ModelIdentifier {
    /// Create a new `ModelIdentifier`
    #[must_use]
    pub const fn new(id: Option<Uuid>, name: Option<String>) -> Self {
        Self { id, name }
    }

    /// Create a new `ModelIdentifier` from the database column
    #[must_use]
    pub const fn from_row(id: Uuid, name: String) -> Self {
        Self::new(Some(id), Some(name))
    }
}
