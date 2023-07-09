//! Cultivar model impls
#![allow(dead_code, clippy::missing_const_for_fn)]

use serde::Serialize;

use crate::{services::produce::harvest::models::HarvestIndex, types::ModelID};

/// A `Vec` of cultivars
pub type CultivarList = Vec<CultivarIndex>;

/// The model representing a row in the `cultivars` database table.
///
/// Returned by `cultivar_detail` handler.
#[derive(Debug, Clone, Serialize)]
pub struct Cultivar {
    pub id: ModelID,
    pub name: String,
    pub category: String,
    pub image: Option<String>,
    pub harvests: Option<Vec<HarvestIndex>>,
}

impl Cultivar {
    /// Creates a new Cultivar from the database row
    #[must_use]
    pub fn from_row(
        id: ModelID,
        name: String,
        category: String,
        image: Option<String>,
        harvests: Option<Vec<HarvestIndex>>,
    ) -> Self {
        Self {
            id,
            name,
            category,
            image,
            harvests,
        }
    }
}

/// A type returned by `cultivar_list` handler.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CultivarIndex {
    pub id: ModelID,
    pub name: String,
    pub image: Option<String>,
    pub harvest_count: u64,
}

impl CultivarIndex {
    /// Creates a new Cultivar from the database row
    #[allow(clippy::cast_sign_loss)]
    #[must_use]
    pub fn from_row(
        id: ModelID,
        name: String,
        image: Option<String>,
        harvest_count: Option<i64>,
    ) -> Self {
        Self {
            id,
            name,
            image,
            harvest_count: harvest_count.unwrap_or(0) as u64,
        }
    }
}
