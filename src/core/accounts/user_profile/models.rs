//! User Profile model impls

use serde::Serialize;
use time::Date;

use crate::{
    accounts::user::models::UserIndex, services::farmers::farm::models::Farm, types::ModelID,
};

/// The model representing a row in the `user_profiles` database table.
///
/// Returned by `user_profile` handler.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub user: UserIndex,
    pub about: String,
    pub lives_at: Option<String>,
    pub date_joined: Date,
    pub farms: Option<Vec<Farm>>,
}

impl UserProfile {
    /// Creates a new `UserProfile` from the database row
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn from_row(
        user_id: ModelID,
        first_name: String,
        last_name: Option<String>,
        about: String,
        lives_at: Option<String>,
        photo: Option<String>,
        date_joined: Date,
        farms: Option<Vec<Farm>>,
    ) -> Self {
        Self {
            user: UserIndex::from_row(user_id, first_name, last_name, photo),
            about,
            lives_at,
            date_joined,
            farms,
        }
    }
}
