#![allow(dead_code, clippy::missing_const_for_fn)]

use serde::Serialize;
use time::{Date, OffsetDateTime};

use crate::{
    core::types::{price::Price, ModelID},
    core::{accounts::user::models::UserIndex, types::ModelIdentifier},
    services::produce::harvest::models::HarvestLocation,
};

/// Return by admin's `harvest detail` handler
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HarvestAdmin {
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
    pub is_available: bool,
    pub updated_at: Option<OffsetDateTime>,
    pub finished_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub location: HarvestLocation,
}

impl HarvestAdmin {
    /// Creates a new `HarvestAdmin` from the database row
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn from_row(
        id: ModelID,
        cultivar_id: ModelID,
        cultivar_name: String,
        farm_id: ModelID,
        farm_name: String,
        farm_owner_id: ModelID,
        farm_owner_first_name: String,
        farm_owner_last_name: Option<String>,
        farm_owner_photo: Option<String>,
        price: serde_json::Value,
        r#type: Option<String>,
        description: Option<String>,
        images: Option<Vec<String>>,
        available_at: Date,
        is_available: bool,
        updated_at: Option<OffsetDateTime>,
        finished_at: Option<OffsetDateTime>,
        created_at: OffsetDateTime,
        location_id: ModelID,
        place_name: String,
        region: Option<String>,
        country: String,
        coords: Option<serde_json::Value>,
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
            is_available,
            updated_at,
            finished_at,
            created_at,
            location: HarvestLocation::from_row(location_id, place_name, region, country, coords),
        }
    }
}
