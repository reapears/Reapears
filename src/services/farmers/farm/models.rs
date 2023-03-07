//! Farm model impls
#![allow(dead_code, clippy::missing_const_for_fn)]

use serde::Serialize;
use time::Date;
use uuid::Uuid;

use crate::{
    core::accounts::user::models::UserIndex,
    services::farmers::location::models::{Location, LocationList},
};

/// A `Vec` of farms
pub type FarmList = Vec<FarmIndex>;

/// The model representing a row in the `farms` database table.
///
/// Returned by `farm_detail` handler.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Farm {
    pub id: Uuid,
    pub name: String,
    pub owner: UserIndex,
    pub registered_on: Date,
    pub locations: Vec<Location>,
}

impl Farm {
    /// Creates a new `Farm` from the database row
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn from_row(
        id: Uuid,
        name: String,
        locations: Vec<Location>,
        registered_on: Date,
        owner_id: Uuid,
        owner_first_name: String,
        owner_last_name: Option<String>,
        owner_photo: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            owner: UserIndex::from_row(owner_id, owner_first_name, owner_last_name, owner_photo),
            locations,
            registered_on,
        }
    }
}

/// A type returned by `farm_list` handler.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FarmIndex {
    pub id: Uuid,
    pub name: String,
    pub owner: UserIndex,
    pub locations: LocationList,
}

impl FarmIndex {
    /// Creates a new `FarmIndex` from the database row
    #[must_use]
    pub fn from_row(
        id: Uuid,
        name: String,
        locations: LocationList,
        owner_id: Uuid,
        owner_first_name: String,
        owner_last_name: Option<String>,
        owner_photo: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            owner: UserIndex::from_row(owner_id, owner_first_name, owner_last_name, owner_photo),
            locations,
        }
    }
}

// /// Filters for Farms
// #[derive(Debug, Clone)]
// pub enum FarmFilter {
//     // farm
//     NameEq(Vec<String>),
//     NameNe(Vec<String>),
//     RegisteredOnLt(Date),
//     RegisteredOnEq(Vec<Date>),
//     RegisteredOnNe(Vec<Date>),
//     RegisteredOnGt(Date),

//     // location
//     PlaceNameEq(Vec<String>),
//     PlaceNameNe(Vec<String>),
//     RegionEq(Vec<String>),
//     RegionNe(Vec<String>),
//     CountryEq(Vec<String>),
//     CountryNe(Vec<String>),

//     // owner
//     OwnerIdEq(Vec<Uuid>),
//     OwnerIdNe(Vec<Uuid>),
//     OwnerFirstNameEq(Vec<String>),
//     OwnerFirstNameNe(Vec<String>),
//     OwnerLastNameEq(Vec<String>),
//     OwnerLastNameNe(Vec<String>),
//     OwnerFullNameEq(Vec<String>),
//     OwnerFullNameNe(Vec<String>),

//     OwnerGenderEq(Vec<String>),
//     OwnerGenderNe(Vec<String>),
//     OwnerDateOfBirthLt(Date),
//     OwnerDateOfBirthEq(Vec<Date>),
//     OwnerDateOfBirthNe(Vec<Date>),
//     OwnerDateOfBirthGt(Date),

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
