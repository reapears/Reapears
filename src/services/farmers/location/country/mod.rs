//! Location country model impls

pub mod db;
pub mod forms;
pub mod handlers;

use uuid::Uuid;
use serde::Serialize;

/// A `Vec` of country
pub type CountryList = Vec<Country>;

/// The model representing a row in the `countries` database table.
#[derive(Debug, Clone, Serialize)]
pub struct Country {
    pub id: Uuid,
    pub name: String,
}

impl Country {
    /// Creates a new Location country from the database row
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn from_row(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}
