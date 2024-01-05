//! Harvest forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, FromRequestParts, Json, Request},
};
use serde::Deserialize;
use time::{Date, OffsetDateTime};

use crate::{
    auth::FarmerUser,
    endpoint::{
        validators::{TransformString, ValidateString},
        EndpointRejection, EndpointResult,
    },
    server::state::ServerState,
    services::farmers::location::permissions::check_user_owns_location,
    types::{price::Price, ModelID},
};

use helpers::{validate_harvest_date, validate_price};

use super::permissions::check_user_can_update_harvest;

/// Harvest create form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarvestCreateForm {
    pub location_id: String,
    pub cultivar_id: String,
    pub price: Price,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub harvest_date: Option<Date>,
}

/// Harvest create form cleaned data
#[derive(Debug, Clone)]
pub struct HarvestInsertData {
    pub id: ModelID,
    pub location_id: ModelID,
    pub cultivar_id: ModelID, //
    pub price: serde_json::Value,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub harvest_date: Date,
    pub created_at: OffsetDateTime,
}

#[allow(clippy::fallible_impl_from)]
impl From<HarvestCreateForm> for HarvestInsertData {
    fn from(form: HarvestCreateForm) -> Self {
        let created_at = OffsetDateTime::now_utc();
        let harvest_date = form.harvest_date.unwrap_or_else(|| created_at.date());
        Self {
            id: ModelID::new(),
            location_id: ModelID::from_str_unchecked(&form.location_id),
            cultivar_id: ModelID::from_str_unchecked(&form.cultivar_id),
            price: serde_json::to_value(form.price).unwrap(),
            r#type: form.r#type,
            description: form.description,
            harvest_date,
            created_at,
        }
    }
}

impl HarvestCreateForm {
    /// Validates harvest form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.location_id.validate_id("Invalid location id")?;
        self.cultivar_id.validate_id("Invalid cultivar id")?;
        validate_price(&self.price)?;

        if let Some(ref type_) = self.r#type {
            type_.validate_len(0, 32, "Harvest type must be at most 32 characters")?;
        }

        if let Some(ref desc) = self.description {
            desc.validate_len(0, 32, "Harvest description must be at most 512 characters")?;
        }

        if let Some(harvest_date) = self.harvest_date {
            validate_harvest_date(harvest_date)?;
        }

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.r#type = self.r#type.as_ref().map(|type_| type_.clean());
        self.description = self.description.as_ref().map(|desc| desc.clean());
    }

    ///  Validate a user has the permissions to create a harvest
    async fn authorize_request(
        state: &ServerState,
        user: FarmerUser,
        location_id: ModelID,
    ) -> EndpointResult<()> {
        // Validate the location belongs to the users's farm
        check_user_owns_location(user.id(), location_id, state.database()).await
    }
}

#[async_trait]
impl FromRequest<ServerState> for HarvestCreateForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let (mut parts, body) = req.into_parts();
        let user = { FarmerUser::from_parts(&mut parts, state).await? };
        let Json(mut harvest) =
            Json::<Self>::from_request(Request::from_parts(parts, body), state).await?;

        // Validate form fields
        harvest.validate()?;

        // Authorize request
        let location_id = ModelID::from_str_unchecked(harvest.location_id.as_str());
        Self::authorize_request(state, user, location_id).await?;

        Ok(harvest)
    }
}

// ===== Update form impls =====

/// Harvest update form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HarvestUpdateForm {
    pub location_id: String,
    pub cultivar_id: String,
    pub price: Price,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub harvest_date: Option<Date>,
}

/// Harvest update form cleaned data
#[derive(Debug, Clone)]
pub struct HarvestUpdateData {
    pub location_id: ModelID,
    pub cultivar_id: ModelID,
    pub price: serde_json::Value,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub harvest_date: Option<Date>,
    pub updated_at: OffsetDateTime,
}

#[allow(clippy::fallible_impl_from)]
impl From<HarvestUpdateForm> for HarvestUpdateData {
    fn from(form: HarvestUpdateForm) -> Self {
        Self {
            location_id: ModelID::from_str_unchecked(form.location_id),
            cultivar_id: ModelID::from_str_unchecked(form.cultivar_id),
            price: serde_json::to_value(form.price).unwrap(),
            r#type: form.r#type,
            description: form.description,
            harvest_date: form.harvest_date,
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

impl HarvestUpdateForm {
    /// Validates harvest form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        //Clean data
        self.clean_data();

        self.location_id.validate_id("Invalid location id")?;
        self.cultivar_id.validate_id("Invalid cultivar id")?;
        validate_price(&self.price)?;

        if let Some(ref type_) = self.r#type {
            type_.validate_len(0, 32, "Harvest type must be at most 32 characters")?;
        }

        if let Some(ref desc) = self.description {
            desc.validate_len(0, 32, "Harvest description must be at most 512 characters")?;
        }

        if let Some(harvest_date) = self.harvest_date {
            validate_harvest_date(harvest_date)?;
        }

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.r#type = self.r#type.as_ref().map(|type_| type_.clean());
        self.description = self.description.as_ref().map(|desc| desc.clean());
    }

    ///  Validate a user has the permissions to update a harvest
    async fn authorize_request(
        state: &ServerState,
        user: FarmerUser,
        harvest_id: ModelID,
        location_id: ModelID,
    ) -> EndpointResult<()> {
        let db = state.database();
        check_user_can_update_harvest(user.id(), location_id, harvest_id, db).await
    }
}

#[async_trait]
impl FromRequest<ServerState> for HarvestUpdateForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let (mut parts, body) = req.into_parts();
        let user = { FarmerUser::from_parts(&mut parts, state).await? };
        let harvest_id = { ModelID::from_request_parts(&mut parts, state).await? };
        let Json(mut harvest) =
            Json::<Self>::from_request(Request::from_parts(parts, body), state).await?;

        // Validate form fields
        harvest.validate()?;

        // Authorize request
        let location_id = ModelID::from_str_unchecked(harvest.location_id.as_str());
        Self::authorize_request(state, user, harvest_id, location_id).await?;

        Ok(harvest)
    }
}

// ===== Helpers =====

mod helpers {
    use crate::{
        core::types::price::Price,
        endpoint::{EndpointRejection, EndpointResult},
        // types::ModelID,
    };
    use time::{Date, OffsetDateTime};

    /// Validate harvest `harvest_date` date is not a past date
    pub fn validate_harvest_date(date: Date) -> EndpointResult<()> {
        if date < OffsetDateTime::now_utc().date() {
            return Err(EndpointRejection::BadRequest(
                "Harvesting date cannot be a past date.".into(),
            ));
        }
        Ok(())
    }

    /// Validate harvest price, amount cannot be zero
    pub fn validate_price(price: &Price) -> EndpointResult<()> {
        if price.amount < 0.into() {
            return Err(EndpointRejection::BadRequest(
                "Harvest price cannot be zero.".into(),
            ));
        }
        Ok(())
    }

    // /// Validates harvest `id` exists
    // #[allow(dead_code)]
    // pub async fn validate_harvest_id(id: ModelID, db: DatabaseConnection) -> EndpointResult<()> {
    //     match sqlx::query!(
    //         r#"
    //             SELECT EXISTS(
    //                 SELECT 1 FROM services.active_harvests harvest
    //                 WHERE harvest.id = $1
    //             ) AS "exists!"
    //         "#,
    //         id.0
    //     )
    //     .fetch_one(&db.pool)
    //     .await
    //     {
    //         // Returns ok is the harvest id exists
    //         Ok(row) => {
    //             if row.exists {
    //                 Ok(())
    //             } else {
    //                 tracing::error!("Harvest id: '{}' does not exists.", id);
    //                 Err(EndpointRejection::BadRequest("Harvest not found.".into()))
    //             }
    //         }
    //         Err(err) => {
    //             tracing::error!("Database error: {}", err);
    //             Err(EndpointRejection::internal_server_error())
    //         }
    //     }
    // }

    // /// Validate `cultivar_id` and `location_id` are records in the database
    // pub async fn validate_cultivar_id_and_location_id_exists(
    //     cultivar_id: ModelID,
    //     location_id: ModelID,
    //     db: DatabaseConnection,
    // ) -> EndpointResult<()> {
    //     match sqlx::query!(
    //         r#"
    //             SELECT
    //                 EXISTS(SELECT 1 FROM services.cultivars WHERE id = $1) AS "cultivar_exists!",
    //                 EXISTS(SELECT 1 FROM services.active_locations WHERE id = $2) AS "location_exists!";
    //         "#,
    //         cultivar_id.0,
    //         location_id.0
    //     )
    //     .fetch_one(&db.pool)
    //     .await
    //     {
    //         Ok(rec) => match (rec.cultivar_exists, rec.location_exists) {
    //             // cultivar and location exists
    //             (true, true) => Ok(()),
    //             // cultivar does not exists
    //             (false, _) => {
    //                 tracing::error!("Cultivar id: '{}' does not exists.", cultivar_id);
    //                 Err(EndpointRejection::BadRequest("Cultivar not found".into()))
    //             }
    //             // location does not exists
    //             (_, false) => {
    //                 tracing::error!("Location id: '{}' does not exists.", location_id);
    //                 Err(EndpointRejection::BadRequest("Location not found".into()))
    //             }
    //         },
    //         Err(err) => {
    //             tracing::error!("Database error: {}", err);
    //             Err(EndpointRejection::internal_server_error())
    //         }
    //     }
    // }
}
