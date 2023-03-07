//! Cultivar impls

pub mod category;
pub mod db;
pub mod forms;
pub mod handlers;
pub mod models;
mod utils;

/// Number of images allowed to be uploaded per cultivar
const CULTIVAR_MAX_IMAGE: usize = 1;
