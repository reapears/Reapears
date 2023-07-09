//! Farm model impls
#![allow(dead_code, clippy::missing_const_for_fn)]

use serde::Serialize;
use time::Date;

use crate::{
    core::accounts::user::models::UserIndex,
    services::farmers::location::models::{Location, LocationList},
    types::ModelID,
};

/// A `Vec` of farms
pub type FarmList = Vec<FarmIndex>;

/// The model representing a row in the `farms` database table.
///
/// Returned by `farm_detail` handler.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Farm {
    pub id: ModelID,
    pub name: String,
    pub owner: UserIndex,
    pub logo: Option<String>,
    pub contact_email: Option<String>,
    pub contact_number: Option<String>,
    pub registered_on: Date,
    pub locations: Vec<Location>,
}

impl Farm {
    /// Creates a new `Farm` from the database row
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn from_row(
        id: ModelID,
        name: String,
        logo: Option<String>,
        contact_email: Option<String>,
        contact_number: Option<String>,
        locations: Vec<Location>,
        registered_on: Date,
        owner_id: ModelID,
        owner_first_name: String,
        owner_last_name: Option<String>,
        owner_photo: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            logo,
            contact_email,
            contact_number,
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
    pub id: ModelID,
    pub logo: Option<String>,
    pub name: String,
    pub owner: UserIndex,
    pub locations: LocationList,
}

impl FarmIndex {
    /// Creates a new `FarmIndex` from the database row
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn from_row(
        id: ModelID,
        name: String,
        logo: Option<String>,
        locations: LocationList,
        owner_id: ModelID,
        owner_first_name: String,
        owner_last_name: Option<String>,
        owner_photo: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            logo,
            owner: UserIndex::from_row(owner_id, owner_first_name, owner_last_name, owner_photo),
            locations,
        }
    }
}
