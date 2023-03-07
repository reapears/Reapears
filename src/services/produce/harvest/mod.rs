//! Harvest impls

pub mod admin;
pub mod db;
pub mod forms;
pub mod handlers;
pub mod models;
pub mod permissions;
mod utils;

pub use utils::{delete_harvest_photos_list, harvest_max_age};

/// Determine for how long the harvest should be on the
/// platform before it's archived.
/// If the harvest has been on the platform for
/// less-than these days it will be deleted.
const HARVEST_MAX_AGE_TO_ARCHIVE: i64 = 4; // days

/// Number of images allowed to be uploaded per harvest
pub const HARVEST_MAX_IMAGE: usize = 5;
