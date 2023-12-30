//! Harvest model impls
#![allow(clippy::missing_const_for_fn)]

use geo::Point;
use serde::Serialize;
use time::{Date, OffsetDateTime};

use crate::{
    core::types::{price::Price, ModelID},
    core::{accounts::user::models::UserIndex, types::ModelIdentifier},
    services::farmers::location,
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
    pub category: String,
    pub cultivar: ModelIdentifier,
    pub farm: HarvestFarm,
    pub farm_owner: UserIndex,
    pub price: Price,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub cultivar_image: Option<String>,
    pub images: Option<Vec<String>>,
    pub harvest_date: Date,
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
        harvest_date: Date,
        created_at: OffsetDateTime,
        cultivar_id: ModelID,
        cultivar_name: String,
        cultivar_category: String,
        cultivar_image: Option<String>,
        location_id: ModelID,
        place_name: String,
        region: Option<String>,
        country: String,
        coords: Option<serde_json::Value>,
        farm_id: ModelID,
        farm_name: String,
        farm_logo: Option<String>,
        farm_contact_number: Option<String>,
        farm_contact_email: Option<String>,
        farm_owner_id: ModelID,
        farm_owner_first_name: String,
        farm_owner_last_name: Option<String>,
        farm_owner_photo: Option<String>,
    ) -> Self {
        Self {
            id,
            name: cultivar_name.clone(),
            category: cultivar_category,
            cultivar: ModelIdentifier::from_row(cultivar_id, cultivar_name),
            farm: HarvestFarm::from_row(
                farm_id,
                farm_name,
                farm_logo,
                farm_contact_number,
                farm_contact_email,
            ),
            farm_owner: UserIndex::from_row(
                farm_owner_id,
                farm_owner_first_name,
                farm_owner_last_name,
                farm_owner_photo,
            ),
            price: Price::from_row(price),
            r#type,
            description,
            cultivar_image,
            images,
            harvest_date,
            created_at: created_at.date(),
            location: HarvestLocation::from_row(location_id, place_name, region, country, coords),
        }
    }
}

/// A type returned by `harvest_list` handler.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HarvestIndex {
    pub id: ModelID,
    pub name: String,
    pub category: String,
    pub farm_name: String,
    pub farm_logo: Option<String>,
    pub price: Price,
    pub images: Option<Vec<String>>,
    pub cultivar_image: Option<String>,
    pub harvest_date: Date,
    pub place_name: String,
    pub region: Option<String>,
    pub country: String,
    pub coords: Option<Point>,

    // This field is for internal use only; it is not sent to the users.
    // it is used for ordering
    #[serde(skip_serializing)]
    pub boost_amount: rust_decimal::Decimal,
}

impl HarvestIndex {
    /// Creates a new `HarvestIndex` from the database row
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn from_row(
        id: ModelID,
        price: serde_json::Value,
        harvest_date: Date,
        images: Option<Vec<String>>,
        cultivar_name: String,
        cultivar_category: String,
        cultivar_image: Option<String>,
        place_name: String,
        region: Option<String>,
        country: String,
        coords: Option<serde_json::Value>,
        farm_name: String,
        farm_logo: Option<String>,
        boost_amount: rust_decimal::Decimal,
    ) -> Self {
        Self {
            id,
            category: cultivar_category,
            name: cultivar_name,
            farm_name,
            farm_logo,
            price: Price::from_row(price),
            images,
            cultivar_image,
            harvest_date,
            country,
            region,
            place_name,
            coords: location::try_into_point(coords),
            boost_amount,
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
    pub coords: Option<Point>,
}

impl HarvestLocation {
    /// Creates a new `HarvestLocation` from the database row
    #[must_use]
    pub fn from_row(
        id: ModelID,
        place_name: String,
        region: Option<String>,
        country: String,
        coords: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id,
            place_name,
            region,
            country,
            coords: location::try_into_point(coords),
        }
    }
}

// A farm a harvest available at
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HarvestFarm {
    pub id: ModelID,
    pub name: String,
    pub logo: Option<String>,
    pub contact_number: Option<String>,
    pub contact_email: Option<String>,
}

impl HarvestFarm {
    /// Creates a new `HarvestFarm` from the database row
    #[must_use]
    pub fn from_row(
        id: ModelID,
        name: String,
        logo: Option<String>,
        contact_number: Option<String>,
        contact_email: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            logo,
            contact_number,
            contact_email,
        }
    }
}
