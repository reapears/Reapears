//! Cultivar-category http handlers impls

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

use super::{forms::CultivarCategoryForm, CategoryList, CultivarCategory};

/// Handles the `GET /cultivars/categories` route.
#[tracing::instrument(skip(db))]
pub async fn cultivar_category_list(
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<CategoryList>> {
    CultivarCategory::records(db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |categories| Ok(Json(categories)),
    )
}

/// Handles the `POST /cultivars/categories` route.
#[tracing::instrument(skip(db, user, form))]
pub async fn cultivar_category_create(
    #[allow(unused_variables)] user: AdminUser,
    State(db): State<DatabaseConnection>,
    form: CultivarCategoryForm,
) -> EndpointResult<StatusCode> {
    CultivarCategory::insert(form.into(), db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |_id| Ok(StatusCode::CREATED),
    )
}

/// Handles the `PUT /cultivars/categories/:category_id` route.
#[tracing::instrument(skip(db, user, form))]
pub async fn cultivar_category_update(
    #[allow(unused_variables)] user: AdminUser,
    category_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: CultivarCategoryForm,
) -> EndpointResult<StatusCode> {
    CultivarCategory::update(category_id, form.into(), db)
        .await
        .map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::OK),
        )
}

/// Handles the `DELETE /cultivars/categories/:category_id` route.
#[tracing::instrument(skip(db, user))]
pub async fn cultivar_category_delete(
    #[allow(unused_variables)] user: AdminUser,
    category_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    CultivarCategory::delete(category_id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |_| Ok(StatusCode::NO_CONTENT),
    )
}
