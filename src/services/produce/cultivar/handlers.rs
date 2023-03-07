//! Cultivar http handlers impls

use axum::{
    extract::{Json, Multipart, Query, State},
    http::StatusCode,
};

use uuid::Uuid;

use crate::{
    auth::CurrentUser,
    endpoint::{EndpointRejection, EndpointResult, ModelId, ValidatedJson},
    files,
    server::state::DatabaseConnection,
    settings::cultivar_uploads_dir,
    types::{ModelIndex, Pagination},
};

use super::{
    forms::{CultivarCreateForm, CultivarUpdateForm},
    models::{Cultivar, CultivarList},
    utils::delete_cultivar_photo,
    CULTIVAR_MAX_IMAGE,
};

/// Handles the `GET /cultivars` route.
#[tracing::instrument(skip(db))]
pub async fn cultivar_list(
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<CultivarList>> {
    let pagination = pg.unwrap_or_default().0;
    Cultivar::records(pagination, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |cultivars| Ok(Json(cultivars)),
    )
}

/// Handles the `GET /cultivars/:cultivar_id` route.
#[tracing::instrument(skip(db))]
pub async fn cultivar_detail(
    ModelId(cultivar_id): ModelId<Uuid>,
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<Cultivar>> {
    let pagination = pg.unwrap_or_default().0;
    Cultivar::find(cultivar_id, Some(pagination), db)
        .await
        .map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |cultivar| {
                cultivar.map_or_else(
                    || Err(EndpointRejection::NotFound("Cultivar not found.".into())),
                    |cultivar| Ok(Json(cultivar)),
                )
            },
        )
}

/// Handles the `POST /cultivars` route.
#[tracing::instrument(skip(db))]
pub async fn cultivar_create(
    current_user: CurrentUser,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<CultivarCreateForm>,
) -> EndpointResult<StatusCode> {
    if current_user.is_staff {
        Cultivar::insert(form.into(), db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_cultivar_id| Ok(StatusCode::CREATED),
        )
    } else {
        Err(EndpointRejection::forbidden())
    }
}

/// Handles the `PUT /cultivars/:cultivar_id` route.
#[tracing::instrument(skip(db))]
pub async fn cultivar_update(
    current_user: CurrentUser,
    ModelId(cultivar_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<CultivarUpdateForm>,
) -> EndpointResult<StatusCode> {
    if current_user.is_staff {
        Cultivar::update(cultivar_id, form.into(), db)
            .await
            .map_or_else(
                |_err| Err(EndpointRejection::internal_server_error()),
                |_| Ok(StatusCode::OK),
            )
    } else {
        Err(EndpointRejection::forbidden())
    }
}

/// Handles the `DELETE /cultivars/:cultivar_id` route.
#[tracing::instrument(skip(db))]
pub async fn cultivar_delete(
    current_user: CurrentUser,
    ModelId(cultivar_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    if current_user.is_staff {
        Cultivar::delete(cultivar_id, db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::NO_CONTENT),
        )
    } else {
        Err(EndpointRejection::forbidden())
    }
}

/// Handles the `GET /cultivars/index` route.
#[tracing::instrument(skip(db))]
pub async fn cultivar_index(
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<ModelIndex>> {
    Cultivar::index(db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |index| Ok(Json(index)),
    )
}

/// Handles the `POST /cultivars/:cultivar_id/photo` route.
#[tracing::instrument(skip(db))]
pub async fn cultivar_image_upload(
    current_user: CurrentUser,
    ModelId(cultivar_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
    multipart: Multipart,
) -> EndpointResult<Json<String>> {
    if current_user.is_staff {
        let (handler, mut uploads) = files::accept_uploads(multipart, CULTIVAR_MAX_IMAGE);
        handler.accept().await?; // Receive file from the client
        if let Some(file) = uploads.files().await {
            // Save an image to the file system
            let saved_to = files::save_image(file, cultivar_uploads_dir()).await?;

            // Save image path to the database
            let (new_image, old_image) = Cultivar::insert_photo(cultivar_id, saved_to, db).await?;

            // Delete old image
            if let Some(old_image) = old_image {
                tokio::spawn(async move { delete_cultivar_photo(&old_image).await });
            }

            Ok(Json(new_image))
        } else {
            Err(EndpointRejection::BadRequest(
                "Cultivar image is not received".into(),
            ))
        }
    } else {
        Err(EndpointRejection::forbidden())
    }
}

/// Handles the `DELETE /cultivars/:cultivar_id/photo` route.
#[tracing::instrument(skip(db))]
pub async fn cultivar_image_delete(
    current_user: CurrentUser,
    ModelId(cultivar_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    if current_user.is_staff {
        Cultivar::delete_photo(cultivar_id, db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::NO_CONTENT),
        )
    } else {
        Err(EndpointRejection::forbidden())
    }
}
