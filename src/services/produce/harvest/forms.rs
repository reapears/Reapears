//! Harvest forms impls

#![allow(clippy::fallible_impl_from)]
use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, FromRequestParts, Json},
    http::Request,
};
use serde::Deserialize;
use time::{Date, OffsetDateTime};
use tokio::task::JoinSet;
use validator::Validate;

use crate::{
    auth::FarmerUser,
    endpoint::{validators::join_validation_tasks, EndpointRejection, EndpointResult},
    server::state::ServerState,
    services::{
        farmers::location::permissions::check_user_owns_location,
        produce::cultivar::forms::validate_cultivar_id,
    },
    types::{price::Price, ModelID},
};

use helpers::{
    validate_available_at, validate_cultivar_id_and_location_id_exists, validate_harvest_id,
    validate_price,
};

use super::permissions::{check_user_can_update_harvest, check_user_owns_harvest};

/// Harvest create form
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct HarvestCreateForm {
    #[validate(length(min = 1, max = 64, message = "Invalid location id"))]
    pub location_id: String,

    #[validate(length(min = 1, max = 64, message = "Invalid cultivar id"))]
    pub cultivar_id: String,

    pub price: Price,

    #[validate(length(min = 1, max = 32))]
    pub r#type: Option<String>,

    #[validate(length(min = 1, max = 512))]
    pub description: Option<String>,

    pub available_at: Option<Date>,
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
    pub available_at: Date,
    pub created_at: OffsetDateTime,
}

impl From<HarvestCreateForm> for HarvestInsertData {
    fn from(form: HarvestCreateForm) -> Self {
        let created_at = OffsetDateTime::now_utc();
        let available_at = form.available_at.unwrap_or_else(|| created_at.date());
        Self {
            id: ModelID::new(),
            location_id: ModelID::from_str_unchecked(&form.location_id),
            cultivar_id: ModelID::from_str_unchecked(&form.cultivar_id),
            price: serde_json::to_value(form.price).unwrap(),
            r#type: form.r#type,
            description: form.description,
            available_at,
            created_at,
        }
    }
}

impl HarvestCreateForm {
    ///  Validate a user has the permissions to create a harvest
    async fn authorize_request(
        state: &ServerState,
        user: FarmerUser,
        location_id: ModelID,
    ) -> EndpointResult<()> {
        // Validate the location belongs to the users's farm
        check_user_owns_location(user.id(), location_id, state.database.clone()).await
    }
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for HarvestCreateForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        let user = { FarmerUser::from_parts(&mut parts, state).await? };
        let Json(harvest) =
            Json::<Self>::from_request(Request::from_parts(parts, body), state).await?;

        let Ok(location_id) = ModelID::try_from(harvest.location_id.as_str()) else{
            return Err(EndpointRejection::BadRequest("Location not found".into()));
        };

        // Authorize request
        Self::authorize_request(state, user, location_id).await?;

        match harvest.validate() {
            Ok(()) => {
                // Validate ids are valid Uuids
                let Ok(cultivar_id) = ModelID::try_from(harvest.cultivar_id.as_str()) else{
                    return Err(EndpointRejection::BadRequest("Cultivar not found".into()));
                };

                let Ok(location_id) = ModelID::try_from(harvest.location_id.as_str()) else{
                    return Err(EndpointRejection::BadRequest("Location not found".into()));
                };

                let db = state.database.clone();
                validate_cultivar_id_and_location_id_exists(cultivar_id, location_id, db).await?;
                validate_price(&harvest.price)?;

                if let Some(available_at) = harvest.available_at {
                    validate_available_at(available_at)?;
                }

                Ok(harvest)
            }
            Err(err) => {
                tracing::error!("Validation error: {}", err);
                Err(EndpointRejection::BadRequest(err.to_string().into()))
            }
        }
    }
}

// ===== Update form impls =====

/// Harvest update form
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct HarvestUpdateForm {
    #[validate(length(min = 1, max = 64, message = "Invalid location id"))]
    pub location_id: Option<String>,

    #[validate(length(min = 1, max = 64, message = "Invalid cultivar id"))]
    pub cultivar_id: Option<String>,

    pub price: Option<Price>,

    #[validate(length(min = 1, max = 32))]
    pub r#type: Option<String>,

    #[validate(length(min = 1, max = 512))]
    pub description: Option<String>,

    pub available_at: Option<Date>,
}

/// Harvest update form cleaned data
#[derive(Debug, Clone)]
pub struct HarvestUpdateData {
    pub location_id: Option<ModelID>,
    pub cultivar_id: Option<ModelID>,
    pub price: serde_json::Value,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub available_at: Option<Date>,
    pub updated_at: OffsetDateTime,
}

impl From<HarvestUpdateForm> for HarvestUpdateData {
    fn from(form: HarvestUpdateForm) -> Self {
        Self {
            location_id: form.location_id.map(ModelID::from_str_unchecked),
            cultivar_id: form.cultivar_id.map(ModelID::from_str_unchecked),
            price: serde_json::to_value(form.price).unwrap(),
            r#type: form.r#type,
            description: form.description,
            available_at: form.available_at,
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

impl HarvestUpdateForm {
    ///  Validate a user has the permissions to update a harvest
    async fn authorize_request(
        state: &ServerState,
        user: FarmerUser,
        harvest_id: ModelID,
        location_id: Option<ModelID>,
    ) -> EndpointResult<()> {
        let db = state.database.clone();
        if let Some(location_id) = location_id {
            // location id is provided, check also the location belongs to current user
            check_user_can_update_harvest(user.id(), location_id, harvest_id, db).await
        } else {
            // Check only the harvest belong to the current user
            check_user_owns_harvest(user.id(), harvest_id, db).await
        }
    }
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for HarvestUpdateForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();
        let user = { FarmerUser::from_parts(&mut parts, state).await? };
        let harvest_id = { ModelID::from_request_parts(&mut parts, state).await? };
        let Json(input) =
            Json::<Self>::from_request(Request::from_parts(parts, body), state).await?;

        let location_id = match input.location_id {
            Some(ref id) => Some(
                ModelID::try_from(id.as_str())
                    .map_err(|_err| EndpointRejection::BadRequest("Location not found".into()))?,
            ),
            None => None,
        };

        // Authorize request
        Self::authorize_request(state, user, harvest_id, location_id).await?;

        match input.validate() {
            Ok(()) => {
                let mut tasks = JoinSet::new();
                let mut task_handlers = Vec::new();
                let db = state.database.clone();

                let harvest_id_handler = tasks.spawn({
                    let db = db.clone();
                    async move { validate_harvest_id(harvest_id, db).await }
                });
                task_handlers.push(harvest_id_handler);

                // Validate cultivar id
                if let Some(ref cultivar_id) = input.cultivar_id {
                    let cultivar_id_handler =
                        tasks.spawn({
                            let Ok(cultivar_id) = ModelID::try_from(cultivar_id.as_str()) else{
                                return Err(EndpointRejection::BadRequest("Cultivar not found".into()));
                            };
                            let db = db.clone();
                            async move { validate_cultivar_id(cultivar_id, db).await }});
                    task_handlers.push(cultivar_id_handler);
                }

                // Wait for tasks to finish
                join_validation_tasks(tasks, &task_handlers).await?;

                // Validate price
                if let Some(ref price) = input.price {
                    validate_price(price)?;
                }

                // Validate available_at date
                if let Some(available_at) = input.available_at {
                    validate_available_at(available_at)?;
                }

                Ok(input)
            }
            Err(err) => {
                tracing::error!("Validation error: {}", err);
                Err(EndpointRejection::BadRequest(err.to_string().into()))
            }
        }
    }
}

// ===== Helpers =====

mod helpers {
    use crate::{
        core::{server::state::DatabaseConnection, types::price::Price},
        endpoint::{EndpointRejection, EndpointResult},
        types::ModelID,
    };
    use time::{Date, OffsetDateTime};

    /// Validate harvest `available_at` date is not a past date
    pub fn validate_available_at(date: Date) -> EndpointResult<()> {
        if date < OffsetDateTime::now_utc().date() {
            return Err(EndpointRejection::BadRequest(
                "Available-at date cannot be a past date.".into(),
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

    /// Validates harvest `id` exists
    pub async fn validate_harvest_id(id: ModelID, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.active_harvests harvest
                    WHERE harvest.id = $1
                ) AS "exists!"
            "#,
            id.0
        )
        .fetch_one(&db.pool)
        .await
        {
            // Returns ok is the harvest id exists
            Ok(row) => {
                if row.exists {
                    Ok(())
                } else {
                    tracing::error!("Harvest id: '{}' does not exists.", id);
                    Err(EndpointRejection::BadRequest("Harvest not found.".into()))
                }
            }
            Err(err) => {
                tracing::error!("Database error: {}", err);
                Err(EndpointRejection::internal_server_error())
            }
        }
    }

    /// Validate `cultivar_id` and `location_id` are records in the database
    pub async fn validate_cultivar_id_and_location_id_exists(
        cultivar_id: ModelID,
        location_id: ModelID,
        db: DatabaseConnection,
    ) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT
                    EXISTS(SELECT 1 FROM services.cultivars WHERE id = $1) AS "cultivar_exists!",
                    EXISTS(SELECT 1 FROM services.active_locations WHERE id = $2) AS "location_exists!";
            "#,
            cultivar_id.0,
            location_id.0
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(rec) => match (rec.cultivar_exists, rec.location_exists) {
                // cultivar and location exists
                (true, true) => Ok(()),
                // cultivar does not exists
                (false, _) => {
                    tracing::error!("Cultivar id: '{}' does not exists.", cultivar_id);
                    Err(EndpointRejection::BadRequest("Cultivar not found".into()))
                }
                // location does not exists
                (_, false) => {
                    tracing::error!("Location id: '{}' does not exists.", cultivar_id);
                    Err(EndpointRejection::BadRequest("Location not found".into()))
                }
            },
            Err(err) => {
                tracing::error!("Database error: {}", err);
                Err(EndpointRejection::internal_server_error())
            }
        }
    }
}
