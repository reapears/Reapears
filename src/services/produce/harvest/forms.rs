//! Harvest forms impls

#![allow(clippy::fallible_impl_from)]
use axum::async_trait;
use serde::Deserialize;
use time::{Date, OffsetDateTime};
use tokio::task::JoinSet;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db,
    endpoint::{
        validators::{join_validation_tasks, parse_uuid, unwrap_uuid},
        EndpointRejection, EndpointResult, ModelId, ValidateForm,
    },
    server::state::ServerState,
    services::{
        farmers::location::forms::validate_location_id,
        produce::cultivar::forms::validate_cultivar_id,
    },
    types::price::Price,
};

use helpers::{
    validate_available_at, validate_cultivar_id_and_location_id_exists, validate_harvest_id,
    validate_price,
};

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
    pub id: Uuid,
    pub location_id: Uuid,
    pub cultivar_id: Uuid, //
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
            id: db::model_id(),
            location_id: unwrap_uuid(&form.location_id),
            cultivar_id: unwrap_uuid(&form.cultivar_id),
            price: serde_json::to_value(form.price).unwrap(),
            r#type: form.r#type,
            description: form.description,
            available_at,
            created_at,
        }
    }
}

#[async_trait]
impl ValidateForm<ServerState> for HarvestCreateForm {
    #[tracing::instrument(skip(self, state), name = "Validate HarvestCreateForm")]
    async fn validate_form(
        self,
        state: &ServerState,
        _model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        match self.validate() {
            Ok(()) => {
                // validate ids are valid Uuids
                let cultivar_id = parse_uuid(
                    &self.cultivar_id,
                    "Cultivar not found",
                    "Invalid cultivar id",
                )?;
                let location_id = parse_uuid(
                    &self.location_id,
                    "Location not found",
                    "Invalid location id",
                )?;

                let db = state.database.clone();
                validate_cultivar_id_and_location_id_exists(cultivar_id, location_id, db).await?;
                validate_price(&self.price)?;

                if let Some(available_at) = self.available_at {
                    validate_available_at(available_at)?;
                }

                Ok(self)
            }
            Err(err) => {
                tracing::error!("Validation error: {}", err);
                Err(EndpointRejection::BadRequest(err.to_string().into()))
            }
        }
    }
}

// Update form impls

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
    pub location_id: Option<Uuid>,
    pub cultivar_id: Option<Uuid>,
    pub price: serde_json::Value,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub available_at: Option<Date>,
    pub updated_at: OffsetDateTime,
}

impl From<HarvestUpdateForm> for HarvestUpdateData {
    fn from(form: HarvestUpdateForm) -> Self {
        Self {
            location_id: form.location_id.map(|id| unwrap_uuid(&id)),
            cultivar_id: form.cultivar_id.map(|id| unwrap_uuid(&id)),
            price: serde_json::to_value(form.price).unwrap(),
            r#type: form.r#type,
            description: form.description,
            available_at: form.available_at,
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

#[async_trait]
impl ValidateForm<ServerState> for HarvestUpdateForm {
    /// NB! `location_id` IS NOT VERIFIED IT BELONG TO CURRENT USER
    #[tracing::instrument(skip(self, state), name = "Validate HarvestUpdateForm")]
    async fn validate_form(
        self,
        state: &ServerState,
        model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        match self.validate() {
            Ok(()) => {
                let mut tasks = JoinSet::new();
                let mut task_handlers = Vec::new();
                let db = state.database.clone();
                let harvest_id_conn = db.clone();

                // extract harvest id
                let Some(ModelId(harvest_id)) = model_id else {
                    tracing::error!("Could not extract harvest_id from request");
                    return Err(EndpointRejection::BadRequest("Harvest id not found".into()));
                };
                let harvest_id_handler = tasks
                    .spawn(async move { validate_harvest_id(harvest_id, harvest_id_conn).await });
                task_handlers.push(harvest_id_handler);

                // validate cultivar id
                if let Some(cultivar_id) = self.cultivar_id.clone() {
                    let db = db.clone();
                    let cultivar_id =
                        parse_uuid(&cultivar_id, "Cultivar not found", "Invalid cultivar id")?;
                    let cultivar_id_handler =
                        tasks.spawn(async move { validate_cultivar_id(cultivar_id, db).await });
                    task_handlers.push(cultivar_id_handler);
                }

                // validate location id
                if let Some(location_id) = self.location_id.clone() {
                    let db = db.clone();
                    let location_id =
                        parse_uuid(&location_id, "Location not found", "Invalid location id")?;
                    let location_id_handler =
                        tasks.spawn(async move { validate_location_id(location_id, db).await });
                    task_handlers.push(location_id_handler);
                }

                // Wait for tasks to finish
                join_validation_tasks(tasks, &task_handlers).await?;

                // validate price
                if let Some(ref price) = self.price {
                    validate_price(price)?;
                }
                // validate available_at date
                if let Some(available_at) = self.available_at {
                    validate_available_at(available_at)?;
                }

                Ok(self)
            }
            Err(err) => {
                tracing::error!("Validation error: {}", err);
                Err(EndpointRejection::BadRequest(err.to_string().into()))
            }
        }
    }
}

mod helpers {
    use crate::{
        core::{server::state::DatabaseConnection, types::price::Price},
        endpoint::{EndpointRejection, EndpointResult},
    };
    use time::{Date, OffsetDateTime};
    use uuid::Uuid;

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
    pub async fn validate_harvest_id(id: Uuid, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.active_harvests harvest
                    WHERE harvest.id = $1
                ) AS "exists!"
            "#,
            id
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
        cultivar_id: Uuid,
        location_id: Uuid,
        db: DatabaseConnection,
    ) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT
                    EXISTS(SELECT 1 FROM services.cultivars WHERE id = $1) AS "cultivar_exists!",
                    EXISTS(SELECT 1 FROM services.active_locations WHERE id = $2) AS "location_exists!";
            "#,
            cultivar_id,
            location_id
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
