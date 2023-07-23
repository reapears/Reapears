//! Farm location impls

pub mod admin;
pub mod country;
pub mod db;
pub mod forms;
pub mod handlers;
pub mod models;
pub mod permissions;
pub mod region;
mod utils;

/// Determine if a location will be deleted or not
///
/// If the number of farm's locations is equal to this
/// number the location will not be deleted.
const LOCATION_MIN_COUNT_TO_DELETE: i64 = 1;
pub use models::try_into_point;
