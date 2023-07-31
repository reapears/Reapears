//! Cultivar-category http handlers impls

use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use crate::{
    auth::AdminUser, endpoint::EndpointResult, server::state::DatabaseConnection, types::ModelID,
};

use super::{forms::CultivarCategoryForm, CategoryList, CultivarCategory};

/// Handles the `GET /cultivars/categories` route.
#[tracing::instrument(skip(db))]
pub async fn cultivar_category_list(
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<CategoryList>> {
    let categories = CultivarCategory::records(db).await?;
    Ok(Json(categories))
}

/// Handles the `POST /cultivars/categories` route.
#[tracing::instrument(skip(db, form))]
pub async fn cultivar_category_create(
    _: AdminUser,
    State(db): State<DatabaseConnection>,
    form: CultivarCategoryForm,
) -> EndpointResult<StatusCode> {
    CultivarCategory::insert(form.into(), db).await?;
    Ok(StatusCode::CREATED)
}

/// Handles the `PUT /cultivars/categories/:category_id` route.
#[tracing::instrument(skip(db, form))]
pub async fn cultivar_category_update(
    _: AdminUser,
    category_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: CultivarCategoryForm,
) -> EndpointResult<StatusCode> {
    CultivarCategory::update(category_id, form.into(), db).await?;
    Ok(StatusCode::OK)
}

/// Handles the `DELETE /cultivars/categories/:category_id` route.
#[tracing::instrument(skip(db,))]
pub async fn cultivar_category_delete(
    _: AdminUser,
    category_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    CultivarCategory::delete(category_id, db).await?;
    Ok(StatusCode::NO_CONTENT)
}
