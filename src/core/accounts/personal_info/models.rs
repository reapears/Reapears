//! User personal info model impls

use serde::Serialize;
use time::Date;

use crate::types::ModelID;

/// User personal infos model
///
/// Returned by `user_personal_info` handler.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalInfo {
    pub id: ModelID,
    pub first_name: String,
    pub last_name: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<Date>,
    pub email: String,
    pub phone: Option<String>,
    pub date_joined: Date,
}

impl PersonalInfo {
    /// Creates a new `PersonalInfo` from the database row
    #[allow(clippy::too_many_arguments, clippy::missing_const_for_fn)]
    #[must_use]
    pub fn from_row(
        id: ModelID,
        first_name: String,
        last_name: Option<String>,
        gender: Option<String>,
        date_of_birth: Option<Date>,
        email: String,
        phone: Option<String>,
        date_joined: Date,
    ) -> Self {
        Self {
            id,
            first_name,
            last_name,
            gender,
            date_of_birth,
            // government_id,
            email,
            phone,
            date_joined,
        }
    }
}
