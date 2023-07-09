#![allow(dead_code, clippy::missing_const_for_fn)]

use geo::Point;
use serde::Serialize;
use time::OffsetDateTime;

use crate::{
    core::types::{ModelID, ModelIdentifier},
    services::produce::harvest::models::HarvestList,
};

use super::models::try_into_point;

/// Return by admin's `location detail` handler
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationAdmin {
    pub id: ModelID,
    pub place_name: String,
    pub farm: ModelIdentifier,
    pub country: String,
    pub region: Option<String>,
    pub coords: Option<Point>,
    pub description: Option<String>,
    pub harvests: Option<HarvestList>,
    pub registered_on: OffsetDateTime,
}

impl LocationAdmin {
    /// Creates a new `LocationAdmin` from the database row
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn from_row(
        id: ModelID,
        place_name: String,
        farm_id: ModelID,
        farm_name: String,
        region: Option<String>,
        country: String,
        coords: Option<serde_json::Value>,
        description: Option<String>,
        harvest_list: Option<HarvestList>,
        registered_on: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            place_name,
            farm: ModelIdentifier::from_row(farm_id, farm_name),
            country,
            region,
            coords: try_into_point(coords),
            description,
            harvests: harvest_list,
            registered_on,
        }
    }
}
