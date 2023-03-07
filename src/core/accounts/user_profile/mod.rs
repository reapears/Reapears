//! User-profile impls

pub mod db;
pub mod forms;
pub mod handlers;
pub mod models;
mod utils;

pub use utils::delete_user_photo;

/// Number of profile photos allowed per user
pub const USER_MAX_PROFILE_PHOTO: usize = 1;
