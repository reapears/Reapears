//! Pagination impl

use serde::{Deserialize, Serialize};

const PAGINATION_PAGE_DEFAULT: u64 = 1;
const PAGINATION_LIMIT_DEFAULT: u64 = 20;

/// Configure the offset and limit of
/// the rows returned from database
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pagination {
    /// Max number of records to be returned
    pub limit: u64,
    pub page: u64,
}

impl Pagination {
    /// Creates a new `Pagination`
    #[must_use]
    pub const fn new(page: u64, limit: u64) -> Self {
        Self { limit, page }
    }

    /// Returns the number of records to be skipped
    #[must_use]
    pub const fn offset(&self) -> u64 {
        // Subtract 1 from page, so we don't skip the first record
        match self.page.checked_sub(1) {
            Some(n) => n * self.limit,
            None => 0,
        }
    }

    /// Return the offset and limit numbers
    #[allow(clippy::cast_possible_wrap)]
    #[must_use]
    pub const fn offset_limit(&self) -> (i64, i64) {
        (self.offset() as i64, self.limit as i64)
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: PAGINATION_PAGE_DEFAULT,
            limit: PAGINATION_LIMIT_DEFAULT,
        }
    }
}
