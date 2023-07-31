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

pub use models::try_into_point;
