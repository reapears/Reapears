//! Location model impls
#![allow(dead_code, clippy::missing_const_for_fn)]

use geo::Point;
use serde::Serialize;
use uuid::Uuid;

use crate::{core::types::ModelIdentifier, services::produce::harvest::models::HarvestList};

/// A `Vec` of locations
pub type LocationList = Vec<LocationIndex>;

/// The model representing a row in the `locations` database table.
///
/// Returned by `location_detail` handler.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub id: Uuid,
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
        id: Uuid,
        place_name: String,
        region: Option<String>,
        country: String,
        coords: Option<serde_json::Value>,
        description: Option<String>,
        farm_id: Uuid,
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
    pub id: Uuid,
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
        id: Uuid,
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

// /// Filters for Locations
// #[derive(Debug, Clone)]
// pub enum LocationFilter {
//     PlaceNameEq(Vec<String>),
//     PlaceNameNe(Vec<String>),
//     RegionEq(Vec<String>),
//     RegionNe(Vec<String>),
//     CountryEq(Vec<String>),
//     CountryNe(Vec<String>),

//     // by farm
//     FarmIdEq(Vec<Uuid>),
//     FarmIdNe(Vec<Uuid>),
//     FarmNameEq(Vec<String>),
//     FarmNameNe(Vec<String>),

//     // harvest
//     HarvestNameEq(Vec<String>),
//     HarvestNameNe(Vec<String>),
//     HarvestCategoryEq(Vec<String>),
//     HarvestCategoryNe(Vec<String>),

//     HarvestCountLt(i64),
//     HarvestCountEq(Vec<i64>),
//     HarvestCountNe(Vec<i64>),
//     HarvestCountGt(i64),

//     HarvestCreatedAtLt(Date),
//     HarvestCreatedAtEq(Vec<Date>),
//     HarvestCreatedAtNe(Vec<Date>),
//     HarvestCreatedAtGt(Date),

//     HarvestAvailableAtLt(Date),
//     HarvestAvailableAtEq(Vec<Date>),
//     HarvestAvailableAtNe(Vec<Date>),
//     HarvestAvailableAtGt(Date),
// }

/// Try convert json value to [Point],
#[must_use]
pub fn try_into_point(coords: Option<serde_json::Value>) -> Option<Point> {
    coords.and_then(|value| serde_json::from_value(value).ok())
}
