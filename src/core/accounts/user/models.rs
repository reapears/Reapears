//! User models impls

use crate::types::ModelID;
use serde::Serialize;

/// A `Vec` of users
pub type UserList = Vec<UserIndex>;

/// The model representing a row in the `users` database table.
#[derive(Debug, Clone, Serialize)]
pub struct User;

/// A type returned by `user_list` handler.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIndex {
    pub id: ModelID,
    pub full_name: String,
    pub photo: Option<String>,
}

impl UserIndex {
    #[must_use]
    /// Creates a new `UserIndex` from the database row
    pub fn from_row(
        id: ModelID,
        first_name: String,
        last_name: Option<String>,
        photo: Option<String>,
    ) -> Self {
        let full_name = concat_names(first_name, last_name);
        Self {
            id,
            full_name,
            photo,
        }
    }
}

// Concatenate two names together
#[allow(clippy::needless_pass_by_value)]
fn concat_names(first_name: String, last_name: Option<String>) -> String {
    let last_name = last_name.unwrap_or_default();
    format!("{last_name} {first_name}").trim().to_owned()
}
