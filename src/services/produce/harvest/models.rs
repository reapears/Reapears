//! Harvest model impls
#![allow(dead_code, clippy::missing_const_for_fn)]

use serde::Serialize;
use time::{Date, OffsetDateTime};

use crate::{
    core::types::{price::Price, ModelID},
    core::{accounts::user::models::UserIndex, types::ModelIdentifier},
};

/// A `Vec` of harvests
pub type HarvestList = Vec<HarvestIndex>;

/// The model representing a row in the `harvests` database table.
///
/// Returned by `harvest_detail` handler.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Harvest {
    pub id: ModelID,
    pub name: String,
    pub cultivar: ModelIdentifier,
    pub farm: ModelIdentifier,
    pub farm_owner: UserIndex,
    pub price: Price,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub images: Option<Vec<String>>,
    pub available_at: Date,
    pub created_at: Date,
    pub location: HarvestLocation,
}

impl Harvest {
    /// Creates a new `Harvest` from the database row
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn from_row(
        id: ModelID,
        price: serde_json::Value,
        r#type: Option<String>,
        description: Option<String>,
        images: Option<Vec<String>>,
        available_at: Date,
        created_at: OffsetDateTime,
        cultivar_id: ModelID,
        cultivar_name: String,
        location_id: ModelID,
        place_name: String,
        region: Option<String>,
        country: String,
        farm_id: ModelID,
        farm_name: String,
        farm_owner_id: ModelID,
        farm_owner_first_name: String,
        farm_owner_last_name: Option<String>,
        farm_owner_photo: Option<String>,
    ) -> Self {
        Self {
            id,
            name: cultivar_name.clone(),
            cultivar: ModelIdentifier::from_row(cultivar_id, cultivar_name),
            farm: ModelIdentifier::from_row(farm_id, farm_name),
            farm_owner: UserIndex::from_row(
                farm_owner_id,
                farm_owner_first_name,
                farm_owner_last_name,
                farm_owner_photo,
            ),
            price: Price::from_row(price),
            r#type,
            description,
            images,
            available_at,
            created_at: created_at.date(),
            location: HarvestLocation::from_row(location_id, place_name, region, country),
        }
    }
}

/// A type returned by `harvest_list` handler.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HarvestIndex {
    pub id: ModelID,
    pub name: String,
    pub farm: String,
    pub price: Price,
    pub images: Option<Vec<String>>,
    pub available_at: Date,
    pub place_name: String,
    pub region: Option<String>,
    pub country: String,
}

impl HarvestIndex {
    /// Creates a new `HarvestIndex` from the database row
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn from_row(
        id: ModelID,
        price: serde_json::Value,
        available_at: Date,
        images: Option<Vec<String>>,
        cultivar_name: String,
        place_name: String,
        region: Option<String>,
        country: String,
        farm: String,
    ) -> Self {
        Self {
            id,
            name: cultivar_name,
            farm,
            price: Price::from_row(price),
            images,
            available_at,
            country,
            region,
            place_name,
        }
    }
}

// A location of harvest available at
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HarvestLocation {
    pub id: ModelID,
    pub place_name: String,
    pub region: Option<String>,
    pub country: String,
}

impl HarvestLocation {
    /// Creates a new `HarvestLocation` from the database row
    #[must_use]
    pub fn from_row(
        id: ModelID,
        place_name: String,
        region: Option<String>,
        country: String,
    ) -> Self {
        Self {
            id,
            place_name,
            region,
            country,
        }
    }
}
