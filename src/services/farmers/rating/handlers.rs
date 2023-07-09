//! `FarmRating` http handlers impls

use axum::{
    extract::{Json, Query, State},
    http::StatusCode,
};

use crate::{
    auth::CurrentUser,
    endpoint::{EndpointRejection, EndpointResult},
    server::state::DatabaseConnection,
    types::ModelID,
    types::Pagination,
};

use super::{
    forms::{FarmRatingCreateForm, FarmRatingUpdateForm},
    models::{FarmRating, FarmRatingList},
    permissions::check_user_owns_rating,
};

/// Handles the `GET /farms/ratings` route.
///
/// Return all ratings ever created
#[tracing::instrument(skip(db))]
pub async fn farm_rating_list(
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<FarmRatingList>> {
    let pagination = pg.unwrap_or_default().0;
    FarmRating::records(pagination, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |farm_ratings| Ok(Json(farm_ratings)),
    )
}

/// Handles the `GET /farms/:farm_id/ratings` route.
#[tracing::instrument(skip(db))]
pub async fn farm_ratings(
    farm_id: ModelID,
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<FarmRatingList>> {
    let pagination = pg.unwrap_or_default().0;
    FarmRating::records_for_farm(farm_id, pagination, db)
        .await
        .map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |farm_ratings| Ok(Json(farm_ratings)),
        )
}

/// Handles the `GET /farms/ratings/rating_id` route.
#[tracing::instrument(skip(db))]
pub async fn farm_rating_detail(
    rating_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<FarmRating>> {
    FarmRating::find(rating_id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |farm_rating| {
            farm_rating.map_or_else(
                || Err(EndpointRejection::NotFound("Farm rating not found".into())),
                |farm_rating| Ok(Json(farm_rating)),
            )
        },
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
    FarmRating::insert(form.data(farm_id, user.id), db)
        .await
        .map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_rating_id| Ok(StatusCode::CREATED),
        )
}

/// Handles the `PUT /farms/ratings/rating_id` route.
#[tracing::instrument(skip(db, user, form))]
pub async fn farm_rating_update(
    #[allow(unused_variables)] user: CurrentUser,
    rating_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: FarmRatingUpdateForm,
) -> EndpointResult<StatusCode> {
    FarmRating::update(rating_id, form.into(), db)
        .await
        .map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::OK),
        )
}

/// Handles the `DELETE /farms/ratings/rating_id` route.
#[tracing::instrument(skip(db, user))]
pub async fn farm_rating_delete(
    user: CurrentUser,
    rating_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    //Review redirection
    let check_permissions = check_user_owns_rating(user.id, rating_id, db.clone());
    match check_permissions.await {
        Ok(()) => FarmRating::delete(rating_id, db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::NO_CONTENT),
        ),

        Err(err) => Err(err),
    }
}
