#![allow(
    dead_code,
    clippy::missing_const_for_fn,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]
use serde::Serialize;
use time::{Date, OffsetDateTime};

use crate::core::{accounts::user::models::UserIndex, types::{ModelIdentifier, ModelID}};

/// The model representing a row in the `locations` database table.
#[derive(Debug, Clone, Serialize)]
pub struct FarmRatingAdmin {
    pub id: ModelID,
    pub grade: u8,
    pub comment: Option<String>,
    pub farm: ModelIdentifier,
    pub author: UserIndex,
    pub update_at: Option<OffsetDateTime>
    pub created_at: Date,
}

impl FarmRatingAdmin {
    /// Creates a new `FarmRatingAdmin` from the database row
    #[allow(clippy::too_many_arguments)]
    pub fn from_row(
        id: ModelID,
        grade: i32,
        comment: Option<String>,
        update_at: Option<OffsetDateTime>
        created_at: Date,
        farm_id: ModelID,
        farm_name: String,
        user_id: ModelID,
        user_first_name: String,
        user_last_name: Option<String>,
        user_photo: Option<String>,
    ) -> Self {
        Self {
            id,
            grade: grade as u8,
            comment,
            farm: ModelIdentifier::from_row(farm_id, farm_name),
            author: UserIndex::from_row(user_id, user_first_name, user_last_name, user_photo),
            update_at,
            created_at,
        }
    }
}