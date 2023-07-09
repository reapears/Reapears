//! Location region model impls

pub mod db;
pub mod forms;
pub mod handlers;

use crate::types::ModelID;
use serde::Serialize;

/// A `Vec` of regions
pub type RegionList = Vec<Region>;

/// The model representing a row in the `regions` database table.
#[derive(Debug, Clone, Serialize)]
pub struct Region {
    pub id: ModelID,
    pub name: String,
}

impl Region {
    /// Creates a new Location region from the database row
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn from_row(id: ModelID, name: String) -> Self {
        Self { id, name }
    }
}
