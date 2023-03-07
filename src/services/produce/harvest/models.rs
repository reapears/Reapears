//! Harvest model impls
#![allow(dead_code, clippy::missing_const_for_fn)]

use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};
use uuid::Uuid;

use crate::{
    core::types::price::Price,
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
    pub id: Uuid,
    pub name: String,
    pub cultivar: ModelIdentifier,
    pub farm: ModelIdentifier,
    pub farm_owner: UserIndex,
    pub price: Price,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub images: Option<HarvestImages>,
    pub available_at: Date,
    pub created_at: Date,
    pub location: HarvestLocation,
}

impl Harvest {
    /// Creates a new `Harvest` from the database row
    #[allow(clippy::too_many_arguments)]
    pub fn from_row(
        id: Uuid,
        price: serde_json::Value,
        r#type: Option<String>,
        description: Option<String>,
        images: Option<serde_json::Value>,
        available_at: Date,
        created_at: OffsetDateTime,
        cultivar_id: Uuid,
        cultivar_name: String,
        location_id: Uuid,
        place_name: String,
        region: Option<String>,
        country: String,
        farm_id: Uuid,
        farm_name: String,
        farm_owner_id: Uuid,
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
            images: images.map(HarvestImages::from_row),
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
    pub id: Uuid,
    pub name: String,
    pub farm: String,
    pub price: Price,
    pub images: Option<HarvestImages>,
    pub available_at: Date,
    pub place_name: String,
    pub region: Option<String>,
    pub country: String,
}

impl HarvestIndex {
    /// Creates a new `HarvestIndex` from the database row
    #[allow(clippy::too_many_arguments)]
    pub fn from_row(
        id: Uuid,
        price: serde_json::Value,
        available_at: Date,
        images: Option<serde_json::Value>,
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
            images: images.map(HarvestImages::from_row),
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
    pub id: Uuid,
    pub place_name: String,
    pub region: Option<String>,
    pub country: String,
}

impl HarvestLocation {
    /// Creates a new `HarvestLocation` from the database row
    #[must_use]
    pub fn from_row(id: Uuid, place_name: String, region: Option<String>, country: String) -> Self {
        Self {
            id,
            place_name,
            region,
            country,
        }
    }
}

/// Harvest image paths
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct HarvestImages {
    pub cover: Option<String>,
    pub n1: Option<String>,
    pub n2: Option<String>,
    pub n3: Option<String>,
    pub n4: Option<String>,
}

impl HarvestImages {
    /// Create new `HarvestImage`
    #[must_use]
    pub fn new(
        cover: Option<String>,
        n1: Option<String>,
        n2: Option<String>,
        n3: Option<String>,
        n4: Option<String>,
    ) -> Self {
        Self {
            cover,
            n1,
            n2,
            n3,
            n4,
        }
    }

    /// Create new `HarvestImage` from json
    #[must_use]
    pub fn from_row(value: serde_json::Value) -> Self {
        serde_json::from_value(value).unwrap_or_default()
    }

    /// Convert `HarvestImages` into `Vec` of `Utf8PathBuf`
    #[must_use]
    pub fn into_paths(self) -> Vec<Utf8PathBuf> {
        let mut paths = Vec::with_capacity(4);
        if let Some(cover) = self.cover {
            paths.push(Utf8PathBuf::from(cover));
        }
        if let Some(n1) = self.n1 {
            paths.push(Utf8PathBuf::from(n1));
        }
        if let Some(n2) = self.n2 {
            paths.push(Utf8PathBuf::from(n2));
        }
        if let Some(n3) = self.n3 {
            paths.push(Utf8PathBuf::from(n3));
        }
        if let Some(n4) = self.n4 {
            paths.push(Utf8PathBuf::from(n4));
        }
        paths
    }
}

// /// Filters for Harvests
// #[derive(Debug, Clone)]
// pub enum HarvestFilter {
//     NameEq(Vec<String>),
//     NameNe(Vec<String>),
//     CategoryEq(Vec<String>),
//     CategoryNe(Vec<String>),

//     CreatedAtLt(Date),
//     CreatedAtEq(Vec<Date>),
//     CreatedAtNe(Vec<Date>),
//     CreatedAtGt(Date),

//     AvailableAtLt(Date),
//     AvailableAtEq(Vec<Date>),
//     AvailableAtNe(Vec<Date>),
//     AvailableAtGt(Date),
//     // by location
//     LocationIdEq(Vec<Uuid>),
//     LocationIdNe(Vec<Uuid>),
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
//     FarmOwnerIdEq(Vec<Uuid>),
//     FarmOwnerIdNe(Vec<Uuid>),
//     FarmOwnerFullNameEq(Vec<String>),
//     FarmOwnerFullNameNe(Vec<String>),
// }
