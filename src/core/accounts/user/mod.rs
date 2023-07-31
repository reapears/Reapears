//! User impls

use time::{Duration, OffsetDateTime};

pub mod db;
pub mod forms;
pub mod handlers;
pub mod models;
mod utils;

/// Gets account confirm token expiry time
fn account_confirm_expiry_time() -> OffsetDateTime {
    OffsetDateTime::now_utc() - Duration::minutes(crate::ACCOUNT_CONFIRM_TOKEN_EXPIRY)
}
