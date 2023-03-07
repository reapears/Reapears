//! Location http handlers impls

use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    auth::CurrentUser,
    endpoint::{EndpointRejection, EndpointResult, ModelId, ValidatedJson},
    server::state::DatabaseConnection,
    services::farmers::farm::permissions::check_user_owns_farm,
    types::{ModelIndex, Pagination},
};

use super::{
    forms::{LocationCreateForm, LocationUpdateForm},
    models::{Location, LocationList},
    permissions::check_user_owns_location,
};

/// Handles the `GET /locations` route.
#[tracing::instrument(skip(db))]
pub async fn location_list(
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<LocationList>> {
    let pagination = pg.unwrap_or_default().0;
    Location::records(pagination, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |locations| Ok(Json(locations)),
    )
}

/// Handles the `GET /locations/:location_id` route.
#[tracing::instrument(skip(db))]
pub async fn location_detail(
    ModelId(id): ModelId<Uuid>,
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<Location>> {
    let pagination = pg.unwrap_or_default().0;
    Location::find(id, Some(pagination), db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |location| {
            location.map_or_else(
                || Err(EndpointRejection::NotFound("Location not found.".into())),
                |location| Ok(Json(location)),
            )
        },
    )
}

/// Handles the `POST /farms/farm_id/locations` route.
#[tracing::instrument(skip(current_user, db, form))]
pub async fn location_create(
    current_user: CurrentUser,
    ModelId(farm_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<LocationCreateForm>,
) -> EndpointResult<StatusCode> {
    let check_permissions = check_user_owns_farm(current_user.id, farm_id, db.clone());
    match check_permissions.await {
        Ok(()) => Location::insert(form.data(farm_id), db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_location_id| Ok(StatusCode::CREATED),
        ),
        Err(err) => Err(err),
    }
}

/// Handles the `PUT /locations/:location_id` route.
#[tracing::instrument(skip(current_user, db, form))]
pub async fn location_update(
    current_user: CurrentUser,
    ModelId(location_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<LocationUpdateForm>,
) -> EndpointResult<StatusCode> {
    let check_permissions = check_user_owns_location(current_user.id, location_id, db.clone());
    match check_permissions.await {
        Ok(()) => Location::update(location_id, form.into(), db)
            .await
            .map_or_else(
                |_err| Err(EndpointRejection::internal_server_error()),
                |_| Ok(StatusCode::OK),
            ),
        Err(err) => Err(err),
    }
}

/// Handles the `DELETE /locations/:location_id` route.
#[tracing::instrument(skip(current_user, db))]
pub async fn location_delete(
    current_user: CurrentUser,
    ModelId(location_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    let check_permissions = check_user_owns_location(current_user.id, location_id, db.clone());
    match check_permissions.await {
        Ok(()) => Location::delete(location_id, db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::NO_CONTENT),
        ),

        Err(err) => Err(err),
    }
}

/// Handles the `GET /locations/countries/:country_id/regions` route.
#[tracing::instrument(skip(db))]
pub async fn region_list(
    ModelId(country_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<ModelIndex>> {
    Location::regions(country_id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |regions| Ok(Json(regions)),
    )
}

/// Handles the `GET /locations/countries` route.
#[tracing::instrument(skip(db))]
pub async fn country_list(
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<ModelIndex>> {
    Location::countries(db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |countries| Ok(Json(countries)),
    )
}
