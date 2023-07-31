//! Location http handlers impls

use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
};

use crate::{
    auth::{AdminUser, FarmerUser},
    endpoint::{EndpointRejection, EndpointResult},
    server::state::DatabaseConnection,
    types::ModelID,
    types::{ModelIndex, Pagination},
};

use super::{
    forms::{LocationCreateForm, LocationUpdateForm},
    models::{Location, LocationList},
    permissions::LocationDeletePermission,
};

/// Handles the `GET /locations` route.
#[tracing::instrument(skip(db))]
pub async fn location_list(
    _: AdminUser,
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<LocationList>> {
    let pagination = pg.unwrap_or_default().0;
    let locations = Location::records(pagination, db).await?;
    Ok(Json(locations))
}

/// Handles the `GET /locations/:location_id` route.
#[tracing::instrument(skip(db))]
pub async fn location_detail(
    id: ModelID,
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<Location>> {
    let pagination = pg.unwrap_or_default().0;
    Location::find(id, Some(pagination), db).await?.map_or_else(
        || Err(EndpointRejection::NotFound("Location not found.".into())),
        |location| Ok(Json(location)),
    )
}

/// Handles the `POST /farms/farm_id/locations` route.
#[tracing::instrument(skip(db, form))]
pub async fn location_create(
    _: FarmerUser,
    farm_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: LocationCreateForm,
) -> EndpointResult<StatusCode> {
    Location::insert(form.data(farm_id), db).await?;
    Ok(StatusCode::CREATED)
}

/// Handles the `PUT /locations/:location_id` route.
#[tracing::instrument(skip(db, form))]
pub async fn location_update(
    _: FarmerUser,
    location_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: LocationUpdateForm,
) -> EndpointResult<StatusCode> {
    Location::update(location_id, form.into(), db).await?;
    Ok(StatusCode::OK)
}

/// Handles the `DELETE /locations/:location_id` route.
#[tracing::instrument(skip(db))]
pub async fn location_delete(
    _: LocationDeletePermission,
    location_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    Location::delete(location_id, db).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Handles the `GET /locations/countries/:country_id/regions` route.
#[tracing::instrument(skip(db))]
pub async fn region_list(
    country_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<ModelIndex>> {
    let regions = Location::regions(country_id, db).await?;
    Ok(Json(regions))
}

/// Handles the `GET /locations/countries` route.
#[tracing::instrument(skip(db))]
pub async fn country_list(
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<ModelIndex>> {
    let countries = Location::countries(db).await?;
    Ok(Json(countries))
}
