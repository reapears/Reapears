//! Cultivar category model impls

pub mod db;
pub mod forms;
pub mod handlers;

use crate::types::ModelID;
use serde::Serialize;

/// A `Vec` of cultivar categories
pub type CategoryList = Vec<CultivarCategory>;

/// The model representing a row in the `cultivar_categories` database table.
#[derive(Debug, Clone, Serialize)]
pub struct CultivarCategory {
    pub id: ModelID,
    pub name: String,
}

impl CultivarCategory {
    /// Creates a new Cultivar category from the database row
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn from_row(id: ModelID, name: String) -> Self {
        Self { id, name }
    }
}
