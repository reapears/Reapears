//! Location forms impls

use axum::async_trait;
use geo::Point;
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
    services::farmers::farm::forms::validate_farm_id,
};

pub use helpers::validate_location_id;
use helpers::{validate_country_id, validate_place_name, validate_region_id, FindPlaceNameBy};

/// Embedded location create form,
/// this form is embedded in `FarmCreateForm`.
/// It differs from `LocationCreateForm` that
/// it does not validate `farm_id` because on farm creation
/// the farm does not exists yet
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LocationEmbeddedForm {
    #[validate(length(min = 1, max = 32))]
    pub place_name: String,

    #[validate(length(min = 1, max = 64, message = "Invalid region id"))]
    pub region_id: String,

    #[validate(length(min = 1, max = 64, message = "Invalid country id"))]
    pub country_id: String,

    #[validate(length(min = 1, max = 256))]
    pub description: Option<String>,

    pub coords: Option<Point>,
}

impl LocationEmbeddedForm {
    /// Converts `Self` into `LocationInsertData`
    #[allow(dead_code)]
    #[must_use]
    pub fn data(self, farm_id: Uuid) -> LocationInsertData {
        LocationInsertData {
            id: db::model_id(),
            farm_id,
            place_name: self.place_name,
            region_id: unwrap_uuid(&self.region_id),
            country_id: unwrap_uuid(&self.country_id),
            description: self.description,
            coords: serde_json::to_value(self.coords).ok(),
            created_at: OffsetDateTime::now_utc().date(),
        }
    }
}

/// Location create form, this form is used
/// when you want to add a new `Farm` location
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LocationCreateForm {
    #[validate(length(min = 1, max = 32))]
    pub place_name: String,

    #[validate(length(min = 1, max = 64, message = "Invalid region id"))]
    pub region_id: String,

    #[validate(length(min = 1, max = 64, message = "Invalid country id"))]
    pub country_id: String,

    #[validate(length(min = 1, max = 256))]
    pub description: Option<String>,

    pub coords: Option<Point>,
}

impl LocationCreateForm {
    /// Convert `Self` into `LocationInsertData`
    #[allow(dead_code)]
    #[must_use]
    pub fn data(self, farm_id: Uuid) -> LocationInsertData {
        LocationInsertData {
            id: db::model_id(),
            farm_id,
            place_name: self.place_name,
            region_id: unwrap_uuid(&self.region_id),
            country_id: unwrap_uuid(&self.country_id),
            description: self.description,
            coords: serde_json::to_value(self.coords).ok(),
            created_at: OffsetDateTime::now_utc().date(),
        }
    }
}

/// Location create cleaned data
#[derive(Debug, Clone)]
pub struct LocationInsertData {
    pub id: Uuid,
    pub farm_id: Uuid,
    pub place_name: String,
    pub region_id: Uuid,
    pub country_id: Uuid,
    pub description: Option<String>,
    pub coords: Option<serde_json::Value>,
    pub created_at: Date,
}

#[async_trait]
impl ValidateForm<ServerState> for LocationEmbeddedForm {
    /// Validates LocationCreateForm by,
    /// validating if the `region_id` and `country_id` exists
    #[tracing::instrument(skip(self, state), name = "Validate-LocationCreateForm")]
    async fn validate_form(
        self,
        state: &ServerState,
        _model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        // TODO: validate user input
        match self.validate() {
            Ok(()) => {
                let mut tasks = JoinSet::new();
                let db = state.database.clone();

                // validate region exists
                let region_id = parse_uuid(
                    &self.region_id,
                    "Location region not found",
                    "Invalid region id",
                )?;
                let region_id_conn = db.clone();
                let region_id_handler =
                    tasks.spawn(async move { validate_region_id(region_id, region_id_conn).await });

                // validate country exists
                let country_id = parse_uuid(
                    &self.country_id,
                    "Location country not found",
                    "Invalid country id",
                )?;
                let country_id_handler =
                    tasks.spawn(async move { validate_country_id(country_id, db).await });

                let task_handlers = [region_id_handler, country_id_handler];

                // Wait for tasks to finish
                join_validation_tasks(tasks, &task_handlers).await?;

                Ok(self)
            }
            Err(err) => {
                tracing::error!("Validation error: {}", err);
                Err(EndpointRejection::BadRequest(err.to_string().into()))
            }
        }
    }
}

#[async_trait]
impl ValidateForm<ServerState> for LocationCreateForm {
    #[tracing::instrument(skip(self, state), name = "Validate LocationCreateForm")]
    async fn validate_form(
        self,
        state: &ServerState,
        model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        match self.validate() {
            Ok(()) => {
                let mut tasks = JoinSet::new();
                let db = state.database.clone();

                // extract farm id
                let Some(ModelId(farm_id)) = model_id else {
                    tracing::error!("Could not extract farm_id from request");
                    return Err(EndpointRejection::BadRequest("Farm id not found".into()));
                };
                let farm_id_conn = db.clone();
                let farm_id_handler =
                    tasks.spawn(async move { validate_farm_id(farm_id, farm_id_conn).await });

                // validate region exists
                let region_id = parse_uuid(
                    &self.region_id,
                    "Location region not found",
                    "Invalid region id",
                )?;
                let region_id_conn = db.clone();
                let region_id_handler =
                    tasks.spawn(async move { validate_region_id(region_id, region_id_conn).await });

                // validate country exists
                let country_id = parse_uuid(
                    &self.country_id,
                    "Location country not found",
                    "Invalid country id",
                )?;
                let country_id_conn = db.clone();
                let country_id_handler = tasks
                    .spawn(async move { validate_country_id(country_id, country_id_conn).await });

                // validate place_name does not exists already
                let place_name = self.place_name.clone();
                let place_name_handler = tasks.spawn(async move {
                    validate_place_name(place_name, FindPlaceNameBy::FarmId(farm_id), db).await
                });

                let task_handlers = [
                    farm_id_handler,
                    region_id_handler,
                    country_id_handler,
                    place_name_handler,
                ];

                // Wait for tasks to finish
                join_validation_tasks(tasks, &task_handlers).await?;

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

/// Location update form
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LocationUpdateForm {
    #[validate(length(min = 1, max = 32))]
    pub place_name: Option<String>,

    #[validate(length(min = 1, max = 64, message = "Invalid region id"))]
    pub region_id: Option<String>,

    #[validate(length(min = 1, max = 64, message = "Invalid country id"))]
    pub country_id: Option<String>,

    #[validate(length(min = 1, max = 256))]
    pub description: Option<String>,

    pub coords: Option<Point>,
}

/// Location update form cleaned data
#[derive(Debug, Clone)]
pub struct LocationUpdateData {
    pub place_name: Option<String>,
    pub region_id: Option<Uuid>,
    pub country_id: Option<Uuid>,
    pub description: Option<String>,
    pub coords: Option<serde_json::Value>,
}

impl From<LocationUpdateForm> for LocationUpdateData {
    fn from(form: LocationUpdateForm) -> Self {
        Self {
            place_name: form.place_name,
            region_id: form.region_id.map(|id| unwrap_uuid(&id)),
            country_id: form.country_id.map(|id| unwrap_uuid(&id)),
            description: form.description,
            coords: serde_json::to_value(form.coords).ok(),
        }
    }
}

#[async_trait]
impl ValidateForm<ServerState> for LocationUpdateForm {
    #[tracing::instrument(skip(self, state), name = "Validate LocationUpdateForm")]
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
                let location_id_conn = db.clone();

                // extract location id
                let Some(ModelId(location_id)) = model_id else {
                    tracing::error!("Could not extract location_id from request");
                    return Err(EndpointRejection::BadRequest("Location id not found".into()));
                };
                let location_id_handler = tasks.spawn(async move {
                    validate_location_id(location_id, location_id_conn).await
                });
                task_handlers.push(location_id_handler);

                // validate region exists
                if let Some(region_id) = self.region_id.clone() {
                    let db = db.clone();
                    let region_id =
                        parse_uuid(&region_id, "Location region not found", "Invalid region id")?;
                    let region_id_handler =
                        tasks.spawn(async move { validate_region_id(region_id, db).await });
                    task_handlers.push(region_id_handler);
                }

                // validate country exists
                if let Some(country_id) = self.country_id.clone() {
                    let db = db.clone();
                    let country_id = parse_uuid(
                        &country_id,
                        "Location country not found",
                        "Invalid country id",
                    )?;
                    let country_id_handler =
                        tasks.spawn(async move { validate_country_id(country_id, db).await });
                    task_handlers.push(country_id_handler);
                }

                // validate place_name is unique
                if let Some(place_name) = self.place_name.clone() {
                    let place_name_handler = tasks.spawn(async move {
                        validate_place_name(
                            place_name,
                            FindPlaceNameBy::LocationId(location_id),
                            db,
                        )
                        .await
                    });
                    task_handlers.push(place_name_handler);
                }

                // Wait for tasks to finish
                join_validation_tasks(tasks, &task_handlers).await?;

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
        core::server::state::DatabaseConnection,
        endpoint::{EndpointRejection, EndpointResult},
    };
    use uuid::Uuid;

    /// Validate `location_id` exists
    pub async fn validate_location_id(id: Uuid, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.active_locations location
                    WHERE location.id = $1
                ) AS "exists!"
            "#,
            id
        )
        .fetch_one(&db.pool)
        .await
        {
            // Returns ok is the location id exists
            Ok(row) => {
                if row.exists {
                    Ok(())
                } else {
                    tracing::error!("Location id: '{}' does not exists.", id);
                    Err(EndpointRejection::BadRequest("Location not found.".into()))
                }
            }
            Err(err) => {
                tracing::error!("Database error: {}", err);
                Err(EndpointRejection::internal_server_error())
            }
        }
    }

    /// Validate `location_id` exists
    pub async fn validate_country_id(id: Uuid, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.countries country
                    WHERE country.id = $1
                ) AS "exists!"
            "#,
            id
        )
        .fetch_one(&db.pool)
        .await
        {
            // Returns ok is the country id exists
            Ok(row) => {
                if row.exists {
                    Ok(())
                } else {
                    tracing::error!("Country id: '{}' does not exists.", id);
                    Err(EndpointRejection::BadRequest("Country not found.".into()))
                }
            }
            Err(err) => {
                tracing::error!("Database error: {}", err);
                Err(EndpointRejection::internal_server_error())
            }
        }
    }

    /// Validate `region_id` exists
    pub async fn validate_region_id(id: Uuid, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.regions region
                    WHERE region.id = $1
                ) AS "exists!"
            "#,
            id
        )
        .fetch_one(&db.pool)
        .await
        {
            // Returns ok is the country id exists
            Ok(row) => {
                if row.exists {
                    Ok(())
                } else {
                    tracing::error!("Region id: '{}' does not exists.", id);
                    Err(EndpointRejection::BadRequest("Region not found.".into()))
                }
            }
            Err(err) => {
                tracing::error!("Database error: {}", err);
                Err(EndpointRejection::internal_server_error())
            }
        }
    }

    pub enum FindPlaceNameBy {
        FarmId(Uuid),
        LocationId(Uuid),
    }

    /// Validate `place_name` is unique to each farm's location
    #[allow(clippy::needless_pass_by_value)]
    pub async fn validate_place_name(
        place_name: String,
        // location_id: Uuid,
        id: FindPlaceNameBy,
        db: DatabaseConnection,
    ) -> EndpointResult<()> {
        fn check_place_name(
            value: &str,
            names: impl Iterator<Item = String>,
        ) -> EndpointResult<()> {
            let place_name = value.to_lowercase();
            for name in names {
                if name.to_lowercase() == place_name {
                    tracing::error!(
                        "Location's place_name belonging to the same farm already exists."
                    );
                    return Err(EndpointRejection::BadRequest(
                        "Location with the same place-name already exists.".into(),
                    ));
                }
            }
            Ok(())
        }

        match id {
            FindPlaceNameBy::FarmId(id) => {
                match sqlx::query!(
                    r#"
                        SELECT location_.place_name AS "place_name!"
                        FROM services.active_locations location_
                        WHERE location_.farm_id = $1
                        "#,
                    id
                )
                .fetch_all(&db.pool)
                .await
                {
                    Ok(records) => {
                        check_place_name(&place_name, records.into_iter().map(|rec| rec.place_name))
                    }
                    Err(err) => {
                        tracing::error!("Database error: {}", err);
                        Err(EndpointRejection::internal_server_error())
                    }
                }
            }
            //
            FindPlaceNameBy::LocationId(id) => {
                match sqlx::query!(
                    r#"
                        SELECT location_.place_name AS "place_name!"
                        FROM services.active_locations location_
                        WHERE location_.farm_id in (
                            SELECT location_.farm_id
                            FROM services.active_locations location_
                            WHERE location_.id = $1
                        )
                        "#,
                    id
                )
                .fetch_all(&db.pool)
                .await
                {
                    Ok(records) => {
                        check_place_name(&place_name, records.into_iter().map(|rec| rec.place_name))
                    }
                    Err(err) => {
                        tracing::error!("Database error: {}", err);
                        Err(EndpointRejection::internal_server_error())
                    }
                }
            }
        }
    }
}
