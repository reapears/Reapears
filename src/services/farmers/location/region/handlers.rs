//! Location region http handlers impls

use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use uuid::Uuid;

use crate::{
    auth::CurrentUser,
    endpoint::{EndpointRejection, EndpointResult, ModelId, ValidatedJson},
    server::state::DatabaseConnection,
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
#[tracing::instrument(skip(db))]
pub async fn region_create(
    current_user: CurrentUser,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<RegionForm>,
) -> EndpointResult<StatusCode> {
    if current_user.is_staff {
        Region::insert(form.into(), db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_id| Ok(StatusCode::CREATED),
        )
    } else {
        Err(EndpointRejection::forbidden())
    }
}

/// Handles the `PUT /locations/countries/regions/region_id` route.
#[tracing::instrument(skip(db))]
pub async fn region_update(
    current_user: CurrentUser,
    ModelId(country_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<RegionForm>,
) -> EndpointResult<StatusCode> {
    if current_user.is_staff {
        Region::update(country_id, form.into(), db)
            .await
            .map_or_else(
                |_err| Err(EndpointRejection::internal_server_error()),
                |_| Ok(StatusCode::OK),
            )
    } else {
        Err(EndpointRejection::forbidden())
    }
}

/// Handles the `DELETE /locations/countries/regions/region_id` route.
#[tracing::instrument(skip(db))]
pub async fn region_delete(
    current_user: CurrentUser,
    ModelId(country_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    if current_user.is_staff {
        Region::delete(country_id, db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::NO_CONTENT),
        )
    } else {
        Err(EndpointRejection::forbidden())
    }
}
