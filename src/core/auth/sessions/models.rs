//! Session models impls

use time::Date;
use uuid::Uuid;

/// The model representing a row in the `sessions` database table.
#[derive(Debug, Clone)]
pub struct Session;

/// User infos used for logging-in
#[derive(Debug, Clone)]
pub struct LoginUser {
    pub id: Uuid,
    pub phc_string: String,
    pub account_locked: bool,
    pub account_locked_reason: Option<String>,
    pub account_locked_until: Option<Date>,
    pub email_verified: bool,
}

impl LoginUser {
    #[must_use]
    /// Creates a new `LoginUser` from the database row
    pub const fn from_row(
        id: Uuid,
        phc_string: String,
        account_locked: bool,
        account_locked_reason: Option<String>,
        account_locked_until: Option<Date>,
        email_verified: bool,
    ) -> Self {
        Self {
            id,
            phc_string,
            account_locked,
            account_locked_reason,
            account_locked_until,
            email_verified,
        }
    }
}
