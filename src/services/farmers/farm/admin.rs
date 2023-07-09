#![allow(dead_code, clippy::missing_const_for_fn)]

use serde::Serialize;
use time::Date;

use crate::{
    core::accounts::user::models::UserIndex, services::farmers::location::admin::LocationAdmin,
    types::ModelID,
};

/// Return by admin's `farm detail` handler
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FarmAdmin {
    pub id: ModelID,
    pub name: String,
    pub owner: UserIndex,
    pub locations: Vec<LocationAdmin>,
    pub registered_on: Date,
}

impl FarmAdmin {
    /// Creates a new `FarmAdmin` from the database row
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn from_row(
        id: ModelID,
        name: String,
        owner_id: ModelID,
        owner_first_name: String,
        owner_last_name: Option<String>,
        owner_photo: Option<String>,
        locations: Vec<LocationAdmin>,
        registered_on: Date,
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
