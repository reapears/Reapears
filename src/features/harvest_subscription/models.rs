//! Harvest subscription models impls

use serde::Serialize;
use time::{Date, OffsetDateTime};

use crate::types::ModelID;

/// A `Vec` of harvest subscriptions
pub type HarvestSubscriptionList = Vec<HarvestSubscription>;

/// The model representing a row in the `harvest_subscriptions` database table.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HarvestSubscription {
    pub id: ModelID,
    pub harvest_id: ModelID,
    pub amount: rust_decimal::Decimal,
    pub expires_at: Date,
    pub created_at: OffsetDateTime,
}

impl HarvestSubscription {
    /// Creates a new `HarvestSubscription` from the database row
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn from_row(
        id: ModelID,
        harvest_id: ModelID,
        amount: rust_decimal::Decimal,
        expires_at: Date,
        created_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            harvest_id,
            amount,
            expires_at,
            created_at,
        }
    }
}
