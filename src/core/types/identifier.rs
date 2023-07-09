//! Model identifier impls

use crate::types::ModelID;
use serde::{Deserialize, Serialize};

/// A `Vec` of identifiers
pub type ModelIndex = Vec<ModelIdentifier>;

/// Unique identifier for models
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ModelIdentifier {
    pub id: Option<ModelID>,
    pub name: Option<String>,
}

impl ModelIdentifier {
    /// Create a new `ModelIdentifier`
    #[must_use]
    pub const fn new(id: Option<ModelID>, name: Option<String>) -> Self {
        Self { id, name }
    }

    /// Create a new `ModelIdentifier` from the database column
    #[must_use]
    pub const fn from_row(id: ModelID, name: String) -> Self {
        Self::new(Some(id), Some(name))
    }
}
