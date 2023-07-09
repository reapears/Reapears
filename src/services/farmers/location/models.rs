//! Location model impls

use geo::Point;
use serde::Serialize;

use crate::{
    core::types::{ModelID, ModelIdentifier},
    services::produce::harvest::models::HarvestList,
};

/// A `Vec` of locations
pub type LocationList = Vec<LocationIndex>;

/// The model representing a row in the `locations` database table.
///
/// Returned by `location_detail` handler.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub id: ModelID,
    pub place_name: String,
    pub farm: ModelIdentifier,
    pub region: Option<String>,
    pub country: String,
    pub coords: Option<Point>,
    pub description: Option<String>,
    pub harvests: Option<HarvestList>,
}

impl Location {
    /// Creates a new `Location` from the database row
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn from_row(
        id: ModelID,
        place_name: String,
        region: Option<String>,
        country: String,
        coords: Option<serde_json::Value>,
        description: Option<String>,
        farm_id: ModelID,
        farm_name: String,
        harvests: Option<HarvestList>,
    ) -> Self {
        Self {
            id,
            place_name,
            farm: ModelIdentifier::from_row(farm_id, farm_name),
            country,
            region,
            coords: try_into_point(coords),
            description,
            harvests,
        }
    }
}

/// A type returned by `location_list` handler.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationIndex {
    pub id: ModelID,
    pub place_name: String,
    pub farm: String,
    pub region: Option<String>,
    pub country: String,
    pub coords: Option<Point>,
    pub harvest_count: u64,
}

impl LocationIndex {
    /// Creates a new `LocationIndex` from the database row
    #[allow(clippy::cast_sign_loss)]
    #[must_use]
    pub fn from_row(
        id: ModelID,
        place_name: String,
        region: Option<String>,
        country: String,
        coords: Option<serde_json::Value>,
        farm: String,
        harvest_count: Option<i64>,
    ) -> Self {
        Self {
            id,
            place_name,
            farm,
            region,
            country,
            coords: try_into_point(coords),
            harvest_count: harvest_count.unwrap_or(0) as u64,
        }
    }
}

/// Try convert json value to [Point],
#[must_use]
pub fn try_into_point(coords: Option<serde_json::Value>) -> Option<Point> {
    coords.and_then(|value| serde_json::from_value(value).ok())
}
