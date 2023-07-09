//! Cultivar http handlers impls

use axum::{
    extract::{Json, Multipart, Query, State},
    http::StatusCode,
};

use crate::{
    auth::AdminUser,
    endpoint::{EndpointRejection, EndpointResult},
    files,
    server::state::DatabaseConnection,
    settings::CULTIVAR_UPLOAD_DIR,
    types::{ModelID, ModelIndex, Pagination},
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
    cultivar_id: ModelID,
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
#[tracing::instrument(skip(db, user, form))]
pub async fn cultivar_create(
    #[allow(unused_variables)] user: AdminUser,
    State(db): State<DatabaseConnection>,
    form: CultivarCreateForm,
) -> EndpointResult<StatusCode> {
    Cultivar::insert(form.into(), db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |_cultivar_id| Ok(StatusCode::CREATED),
    )
}

/// Handles the `PUT /cultivars/:cultivar_id` route.
#[tracing::instrument(skip(db, user, form))]
pub async fn cultivar_update(
    #[allow(unused_variables)] user: AdminUser,
    cultivar_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: CultivarUpdateForm,
) -> EndpointResult<StatusCode> {
    Cultivar::update(cultivar_id, form.into(), db)
        .await
        .map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::OK),
        )
}

/// Handles the `DELETE /cultivars/:cultivar_id` route.
#[tracing::instrument(skip(db, user))]
pub async fn cultivar_delete(
    #[allow(unused_variables)] user: AdminUser,
    cultivar_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    Cultivar::delete(cultivar_id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |_| Ok(StatusCode::NO_CONTENT),
    )
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
#[tracing::instrument(skip(db, user, multipart))]
pub async fn cultivar_image_upload(
    #[allow(unused_variables)] user: AdminUser,
    cultivar_id: ModelID,
    State(db): State<DatabaseConnection>,
    multipart: Multipart,
) -> EndpointResult<Json<String>> {
    let (handler, mut uploads) = files::accept_uploads(multipart, CULTIVAR_MAX_IMAGE);
    handler.accept().await?; // Receive file from the client
    if let Some(file) = uploads.files().await {
        // Save an image to the file system
        let paths = files::save_image(file, CULTIVAR_UPLOAD_DIR).await?;

        // Save image path to the database
        let (path, old_image) = Cultivar::insert_photo(cultivar_id, paths.clone(), db).await?;

        if let Some(old_image) = old_image {
            tokio::spawn(async move { delete_cultivar_photo(&old_image).await });
        }

        Ok(Json(path))
    } else {
        Err(EndpointRejection::BadRequest(
            "Cultivar image is not received".into(),
        ))
    }
}

/// Handles the `DELETE /cultivars/:cultivar_id/photo` route.
#[tracing::instrument(skip(db, user))]
pub async fn cultivar_image_delete(
    #[allow(unused_variables)] user: AdminUser,
    cultivar_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    Cultivar::delete_photo(cultivar_id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |_| Ok(StatusCode::NO_CONTENT),
    )
}
