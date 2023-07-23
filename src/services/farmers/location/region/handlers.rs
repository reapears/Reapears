//! Location region http handlers impls

use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use crate::{
    auth::AdminUser,
    endpoint::{EndpointRejection, EndpointResult},
    server::state::DatabaseConnection,
    types::ModelID,
};

use super::{forms::RegionForm, Region, RegionList};

/// Handles the `GET /locations/countries/:country_id/regions` route.
#[tracing::instrument(skip(db))]
pub async fn region_list(State(db): State<DatabaseConnection>) -> EndpointResult<Json<RegionList>> {
    Region::records(db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |regions| Ok(Json(regions)),
    )
}

/// Handles the `POST /locations/countries/regions` route.
#[tracing::instrument(skip(db, user, form))]
pub async fn region_create(
    #[allow(unused_variables)] user: AdminUser,
    State(db): State<DatabaseConnection>,
    form: RegionForm,
) -> EndpointResult<StatusCode> {
    Region::insert(form.into(), db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |_id| Ok(StatusCode::CREATED),
    )
}

/// Handles the `PUT /locations/countries/regions/region_id` route.
#[tracing::instrument(skip(db, user, form))]
pub async fn region_update(
    #[allow(unused_variables)] user: AdminUser,
    region_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: RegionForm,
) -> EndpointResult<StatusCode> {
    Region::update(region_id, form.into(), db)
        .await
        .map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::OK),
        )
}

/// Handles the `DELETE /locations/countries/regions/region_id` route.
#[tracing::instrument(skip(db, user))]
pub async fn region_delete(
    #[allow(unused_variables)] user: AdminUser,
    region_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    Region::delete(region_id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |_| Ok(StatusCode::NO_CONTENT),
    )
}
