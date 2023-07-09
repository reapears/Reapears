//! Location forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, FromRequestParts, Json},
    http::Request,
};
use geo::Point;
use serde::Deserialize;
use time::{Date, OffsetDateTime};
use tokio::task::JoinSet;
use validator::Validate;

use crate::{
    auth::FarmerUser,
    endpoint::{validators::join_validation_tasks, EndpointRejection, EndpointResult},
    server::state::ServerState,
    services::farmers::farm::forms::validate_farm_id,
    services::farmers::farm::permissions::check_user_owns_farm,
    types::ModelID,
};

use super::permissions::check_user_owns_location;

pub use helpers::{validate_country_id, validate_location_id};
use helpers::{validate_place_name, validate_region_id, FindPlaceNameBy};

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
    pub fn data(self, farm_id: ModelID) -> LocationInsertData {
        LocationInsertData {
            id: ModelID::new(),
            farm_id,
            place_name: self.place_name,
            region_id: ModelID::from_str_unchecked(&self.region_id),
            country_id: ModelID::from_str_unchecked(&self.country_id),
            description: self.description,
            coords: serde_json::to_value(self.coords).ok(),
            created_at: OffsetDateTime::now_utc().date(),
        }
    }

    /// Validate location form inputs
    pub async fn validate_form(&self, state: &ServerState) -> EndpointResult<()> {
        match self.validate() {
            Ok(()) => {
                let mut tasks = JoinSet::new();
                let db = state.database.clone();

                // Validate region exists
                let region_id_handler = tasks.spawn({
                    let Ok(region_id) = ModelID::try_from(self.region_id.as_str()) else{
                        return Err(EndpointRejection::BadRequest("Region not found".into()));
                    };
                    let db = db.clone();
                    async move { validate_region_id(region_id, db).await }
                });

                // Validate country exists
                let country_id_handler = tasks.spawn({
                    let Ok(country_id) = ModelID::try_from(self.country_id.as_str()) else{
                            return Err(EndpointRejection::BadRequest("Country not found".into()));
                    };
                    async move { validate_country_id(country_id, db).await }
                });

                let task_handlers = [region_id_handler, country_id_handler];

                // Wait for tasks to finish
                join_validation_tasks(tasks, &task_handlers).await?;

                Ok(())
            }
            Err(err) => {
                tracing::error!("Validation error: {}", err);
                Err(EndpointRejection::BadRequest(err.to_string().into()))
            }
        }
    }
}

// ===== Location Create form impl =====

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
    pub fn data(self, farm_id: ModelID) -> LocationInsertData {
        LocationInsertData {
            id: ModelID::new(),
            farm_id,
            place_name: self.place_name,
            region_id: ModelID::from_str_unchecked(&self.region_id),
            country_id: ModelID::from_str_unchecked(&self.country_id),
            description: self.description,
            coords: serde_json::to_value(self.coords).ok(),
            created_at: OffsetDateTime::now_utc().date(),
        }
    }
}

/// Location create cleaned data
#[derive(Debug, Clone)]
pub struct LocationInsertData {
    pub id: ModelID,
    pub farm_id: ModelID,
    pub place_name: String,
    pub region_id: ModelID,
    pub country_id: ModelID,
    pub description: Option<String>,
    pub coords: Option<serde_json::Value>,
    pub created_at: Date,
}

impl LocationCreateForm {
    ///  Validate a user has the permissions to crate a location on this farm
    async fn authorize_request(
        user: FarmerUser,
        farm_id: ModelID,
        state: &ServerState,
    ) -> EndpointResult<()> {
        check_user_owns_farm(user.id(), farm_id, state.database.clone()).await
    }
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for LocationCreateForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();
        let user = { FarmerUser::from_parts(&mut parts, state).await? };
        let farm_id = { ModelID::from_request_parts(&mut parts, state).await? };
        let Json(input) =
            Json::<Self>::from_request(Request::from_parts(parts, body), state).await?;

        // Authorize the request
        Self::authorize_request(user, farm_id, state).await?;

        match input.validate() {
            Ok(()) => {
                let mut tasks = JoinSet::new();
                let db = state.database.clone();

                // Validate farm id
                let farm_id_handler = tasks.spawn({
                    let db = db.clone();
                    async move { validate_farm_id(farm_id, db).await }
                });

                // Validate region exists
                let region_id_handler = tasks.spawn({
                    let Ok(region_id) = ModelID::try_from(input.region_id.as_str()) else{
                        return Err(EndpointRejection::BadRequest("Region not found".into()));
                    };
                    let db = db.clone();
                    async move { validate_region_id(region_id, db).await }
                });

                // Validate country exists
                let country_id_handler = tasks.spawn({
                    let Ok(country_id) = ModelID::try_from(input.country_id.as_str()) else{
                        return Err(EndpointRejection::BadRequest("Country not found".into()));
                    };
                    let db = db.clone();
                    async move { validate_country_id(country_id, db).await }
                });

                // Validate placename does not exists already ??
                let place_name_handler = tasks.spawn({
                    let place_name = input.place_name.clone();
                    async move {
                        validate_place_name(place_name, FindPlaceNameBy::FarmId(farm_id), db).await
                    }
                });

                let task_handlers = [
                    farm_id_handler,
                    region_id_handler,
                    country_id_handler,
                    place_name_handler,
                ];

                // Wait for tasks to finish
                join_validation_tasks(tasks, &task_handlers).await?;

                Ok(input)
            }
            Err(err) => {
                tracing::error!("Validation error: {}", err);
                Err(EndpointRejection::BadRequest(err.to_string().into()))
            }
        }
    }
}

// ===== Location Update form impls ======

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
    pub region_id: Option<ModelID>,
    pub country_id: Option<ModelID>,
    pub description: Option<String>,
    pub coords: Option<serde_json::Value>,
}

impl From<LocationUpdateForm> for LocationUpdateData {
    fn from(form: LocationUpdateForm) -> Self {
        Self {
            place_name: form.place_name,
            region_id: form.region_id.map(ModelID::from_str_unchecked),
            country_id: form.country_id.map(ModelID::from_str_unchecked),
            description: form.description,
            coords: serde_json::to_value(form.coords).ok(),
        }
    }
}

impl LocationUpdateForm {
    ///  Validate a user has the permissions to crate a location on this farm
    async fn authorize_request(
        user: FarmerUser,
        location_id: ModelID,
        state: &ServerState,
    ) -> EndpointResult<()> {
        check_user_owns_location(user.id(), location_id, state.database.clone()).await
    }
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for LocationUpdateForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();
        let user = { FarmerUser::from_parts(&mut parts, state).await? };
        let location_id = { ModelID::from_request_parts(&mut parts, state).await? };
        let Json(input) =
            Json::<Self>::from_request(Request::from_parts(parts, body), state).await?;

        // Authorize the request
        Self::authorize_request(user, location_id, state).await?;

        match input.validate() {
            Ok(()) => {
                let mut tasks = JoinSet::new();
                let mut task_handlers = Vec::new();
                let db = state.database.clone();

                // Validate location id
                let location_id_handler = tasks.spawn({
                    let db = db.clone();
                    async move { validate_location_id(location_id, db).await }
                });
                task_handlers.push(location_id_handler);

                // Validate region exists
                if let Some(ref region_id) = input.region_id {
                    let region_id_handler =
                        tasks.spawn({
                            let Ok(region_id) = ModelID::try_from(region_id.as_str()) else{
                                return Err(EndpointRejection::BadRequest("Region not found".into()));
                            };
                            let db = db.clone();
                            async move { validate_region_id(region_id, db).await }});
                    task_handlers.push(region_id_handler);
                }

                // Validate country exists
                if let Some(ref country_id) = input.country_id {
                    let country_id_handler = tasks
                        .spawn({
                            let Ok(country_id) = ModelID::try_from(country_id.as_str()) else{
                                return Err(EndpointRejection::BadRequest("Country not found".into()));
                            };
                            let db = db.clone();
                            async move { validate_country_id(country_id, db).await }});
                    task_handlers.push(country_id_handler);
                }

                // Validate placename is unique
                if let Some(place_name) = input.place_name.clone() {
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
        core::server::state::DatabaseConnection,
        endpoint::{EndpointRejection, EndpointResult},
        types::ModelID,
    };

    /// Validate `location_id` exists
    pub async fn validate_location_id(id: ModelID, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.active_locations location
                    WHERE location.id = $1
                ) AS "exists!"
            "#,
            id.0
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
    pub async fn validate_country_id(id: ModelID, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.countries country
                    WHERE country.id = $1
                ) AS "exists!"
            "#,
            id.0
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
    pub async fn validate_region_id(id: ModelID, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.regions region
                    WHERE region.id = $1
                ) AS "exists!"
            "#,
            id.0
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
        FarmId(ModelID),
        LocationId(ModelID),
    }

    /// Validate `place_name` is unique to each farm's location
    #[allow(clippy::needless_pass_by_value)]
    pub async fn validate_place_name(
        place_name: String,
        // location_id: ModelID,
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
                    id.0
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
                    id.0
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
