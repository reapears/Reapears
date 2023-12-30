//!`HarvestSubscription` forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json, Request},
};
use serde::Deserialize;
use time::{Date, OffsetDateTime};

use crate::{
    endpoint::{validators::ValidateString, EndpointRejection, EndpointResult},
    server::state::ServerState,
    types::ModelID,
};

/// `HarvestSubscription` create form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarvestSubscriptionForm {
    pub harvest_id: String,
    pub amount: rust_decimal::Decimal,
    pub expires_at: Date,
}

/// `HarvestSubscription` insert cleaned data
#[derive(Debug, Clone)]
pub struct HarvestSubscriptionInsertData {
    pub id: ModelID,
    pub harvest_id: ModelID,
    pub amount: rust_decimal::Decimal,
    pub expires_at: Date,
    pub created_at: OffsetDateTime,
}

impl From<HarvestSubscriptionForm> for HarvestSubscriptionInsertData {
    fn from(form: HarvestSubscriptionForm) -> Self {
        Self {
            id: ModelID::new(),
            harvest_id: ModelID::from_str_unchecked(form.harvest_id),
            amount: form.amount,
            expires_at: form.expires_at,
            created_at: OffsetDateTime::now_utc(),
        }
    }
}

/// `HarvestSubscription` update cleaned data
#[derive(Debug, Clone)]
pub struct HarvestSubscriptionUpdateData {
    pub harvest_id: ModelID,
    pub amount: rust_decimal::Decimal,
    pub expires_at: Date,
    pub updated_at: OffsetDateTime,
}

impl From<HarvestSubscriptionForm> for HarvestSubscriptionUpdateData {
    fn from(form: HarvestSubscriptionForm) -> Self {
        Self {
            harvest_id: ModelID::from_str_unchecked(form.harvest_id),
            amount: form.amount,
            expires_at: form.expires_at,
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

impl HarvestSubscriptionForm {
    /// Validates harvest subscription form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        self.harvest_id.validate_id("Invalid harvest id")?;

        Ok(())
    }
}

#[async_trait]
impl FromRequest<ServerState> for HarvestSubscriptionForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut subscription) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        subscription.validate()?;

        Ok(subscription)
    }
}
