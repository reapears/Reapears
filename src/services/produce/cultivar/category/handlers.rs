//! Cultivar-category http handlers impls

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
#[tracing::instrument(skip(db))]
pub async fn cultivar_category_create(
    current_user: CurrentUser,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<CultivarCategoryForm>,
) -> EndpointResult<StatusCode> {
    if current_user.is_staff {
        CultivarCategory::insert(form.into(), db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_id| Ok(StatusCode::CREATED),
        )
    } else {
        Err(EndpointRejection::forbidden())
    }
}

/// Handles the `PUT /cultivars/categories/:category_id` route.
#[tracing::instrument(skip(db))]
pub async fn cultivar_category_update(
    current_user: CurrentUser,
    ModelId(category_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<CultivarCategoryForm>,
) -> EndpointResult<StatusCode> {
    if current_user.is_staff {
        CultivarCategory::update(category_id, form.into(), db)
            .await
            .map_or_else(
                |_err| Err(EndpointRejection::internal_server_error()),
                |_| Ok(StatusCode::OK),
            )
    } else {
        Err(EndpointRejection::forbidden())
    }
}

/// Handles the `DELETE /cultivars/categories/:category_id` route.
#[tracing::instrument(skip(db))]
pub async fn cultivar_category_delete(
    current_user: CurrentUser,
    ModelId(category_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    if current_user.is_staff {
        CultivarCategory::delete(category_id, db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::NO_CONTENT),
        )
    } else {
        Err(EndpointRejection::forbidden())
    }
}
