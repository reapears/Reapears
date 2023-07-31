//! Location region http handlers impls

use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use crate::{
    auth::AdminUser, endpoint::EndpointResult, server::state::DatabaseConnection, types::ModelID,
};

use super::{forms::RegionForm, Region, RegionList};

/// Handles the `GET /locations/countries/:country_id/regions` route.
#[tracing::instrument(skip(db))]
pub async fn region_list(State(db): State<DatabaseConnection>) -> EndpointResult<Json<RegionList>> {
    let regions = Region::records(db).await?;
    Ok(Json(regions))
}

/// Handles the `POST /locations/countries/regions` route.
#[tracing::instrument(skip(db, form))]
pub async fn region_create(
    _: AdminUser,
    State(db): State<DatabaseConnection>,
    form: RegionForm,
) -> EndpointResult<StatusCode> {
    Region::insert(form.into(), db).await?;
    Ok(StatusCode::CREATED)
}

/// Handles the `PUT /locations/countries/regions/region_id` route.
#[tracing::instrument(skip(db, form))]
pub async fn region_update(
    _: AdminUser,
    region_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: RegionForm,
) -> EndpointResult<StatusCode> {
    Region::update(region_id, form.into(), db).await?;
    Ok(StatusCode::OK)
}

/// Handles the `DELETE /locations/countries/regions/region_id` route.
#[tracing::instrument(skip(db,))]
pub async fn region_delete(
    _: AdminUser,
    region_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    Region::delete(region_id, db).await?;
    Ok(StatusCode::NO_CONTENT)
}
