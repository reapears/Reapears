//! `FarmRating` http handlers impls

use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
};

use crate::{
    auth::{AdminUser, CurrentUser},
    endpoint::{EndpointRejection, EndpointResult},
    server::state::DatabaseConnection,
    types::ModelID,
    types::Pagination,
};

use super::{
    forms::{FarmRatingCreateForm, FarmRatingUpdateForm},
    models::{FarmRating, FarmRatingList},
    permissions::FarmRatingOwnershipPermission,
};

/// Handles the `GET /farms/ratings` route.
///
/// Return all ratings ever created
#[tracing::instrument(skip(db))]
pub async fn farm_rating_list(
    _: AdminUser,
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<FarmRatingList>> {
    let pagination = pg.unwrap_or_default().0;
    let farm_ratings = FarmRating::records(pagination, db).await?;
    Ok(Json(farm_ratings))
}

/// Handles the `GET /farms/:farm_id/ratings` route.
#[tracing::instrument(skip(db))]
pub async fn farm_ratings(
    farm_id: ModelID,
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<FarmRatingList>> {
    let pagination = pg.unwrap_or_default().0;
    let farm_ratings = FarmRating::records_for_farm(farm_id, pagination, db).await?;
    Ok(Json(farm_ratings))
}

/// Handles the `GET /farms/ratings/rating_id` route.
#[tracing::instrument(skip(db))]
pub async fn farm_rating_detail(
    rating_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<FarmRating>> {
    FarmRating::find(rating_id, db).await?.map_or_else(
        || Err(EndpointRejection::NotFound("Farm rating not found".into())),
        |farm_rating| Ok(Json(farm_rating)),
    )
}

/// Handles the `POST /farms/:farm_id/ratings` route.
#[tracing::instrument(skip(db, user, form))]
pub async fn farm_rating_create(
    user: CurrentUser,
    farm_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: FarmRatingCreateForm,
) -> EndpointResult<StatusCode> {
    FarmRating::insert(form.data(farm_id, user.id), db).await?;
    Ok(StatusCode::CREATED)
}

/// Handles the `PUT /farms/ratings/rating_id` route.
#[tracing::instrument(skip(db, form))]
pub async fn farm_rating_update(
    rating_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: FarmRatingUpdateForm,
) -> EndpointResult<StatusCode> {
    FarmRating::update(rating_id, form.into(), db).await?;
    Ok(StatusCode::OK)
}

/// Handles the `DELETE /farms/ratings/rating_id` route.
#[tracing::instrument(skip(db))]
pub async fn farm_rating_delete(
    _: FarmRatingOwnershipPermission,
    rating_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    FarmRating::delete(rating_id, db).await?;
    Ok(StatusCode::NO_CONTENT)
}
