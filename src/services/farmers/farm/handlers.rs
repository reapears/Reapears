//! Farm http handlers impls

use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
};

use uuid::Uuid;

use crate::{
    auth::CurrentUser,
    endpoint::{EndpointRejection, EndpointResult, ModelId, ValidatedJson},
    server::state::DatabaseConnection,
    types::{ModelIndex, Pagination},
};

use super::{
    forms::{FarmCreateForm, FarmUpdateForm},
    models::{Farm, FarmList},
    permissions::check_user_owns_farm,
};

/// Handles the `GET /farms` route.
#[tracing::instrument(skip(db))]
pub async fn farm_list(
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<FarmList>> {
    // Available to staff only
    let pagination = pg.unwrap_or_default().0;
    Farm::records(pagination, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |farms| Ok(Json(farms)),
    )
}

/// Handles the `GET /farms/:farm_id` route.
#[tracing::instrument(skip(db))]
pub async fn farm_detail(
    ModelId(farm_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<Farm>> {
    // Available to staff only
    Farm::find(farm_id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |farm| {
            farm.map_or_else(
                || Err(EndpointRejection::NotFound("Farm not found".into())),
                |farm| Ok(Json(farm)),
            )
        },
    )
}

/// Handles the `POST /farms` route.
#[tracing::instrument(skip(current_user, db, form))]
pub async fn farm_create(
    current_user: CurrentUser,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<FarmCreateForm>,
) -> EndpointResult<StatusCode> {
    Farm::insert(form.data(current_user.id), db)
        .await
        .map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_farm_id| Ok(StatusCode::CREATED),
        )
}

/// Handles the `PUT /farms/:farm_id` route.
#[tracing::instrument(skip(current_user, db, form))]
pub async fn farm_update(
    current_user: CurrentUser,
    ModelId(farm_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<FarmUpdateForm>,
) -> EndpointResult<StatusCode> {
    if current_user.is_farmer {
        let check_permissions = check_user_owns_farm(current_user.id, farm_id, db.clone());
        match check_permissions.await {
            Ok(()) => Farm::update(farm_id, form.into(), db).await.map_or_else(
                |_err| Err(EndpointRejection::internal_server_error()),
                |_| Ok(StatusCode::OK),
            ),
            Err(err) => Err(err),
        }
    } else {
        Err(EndpointRejection::forbidden())
    }
}

/// Handles the `DELETE /farms/:farm_id` route.
#[tracing::instrument(skip(current_user, db))]
pub async fn farm_delete(
    current_user: CurrentUser,
    ModelId(farm_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    if current_user.is_farmer {
        let check_permissions = check_user_owns_farm(current_user.id, farm_id, db.clone());
        match check_permissions.await {
            Ok(()) => Farm::delete(farm_id, db).await.map_or_else(
                |_err| Err(EndpointRejection::internal_server_error()),
                |_| Ok(StatusCode::NO_CONTENT),
            ),
            Err(err) => Err(err),
        }
    } else {
        Err(EndpointRejection::forbidden())
    }
}

/// Handles the `GET /farms/:farm_id/locations` route.
#[tracing::instrument(skip(db))]
pub async fn farm_location_index(
    current_user: CurrentUser,
    ModelId(farm_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<ModelIndex>> {
    Farm::location_index(farm_id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |location_index| Ok(Json(location_index)),
    )
}
