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
};

/// Handles the `GET /cultivars` route.
#[tracing::instrument(skip(db))]
pub async fn cultivar_list(
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<CultivarList>> {
    let pagination = pg.unwrap_or_default().0;
    let cultivars = Cultivar::records(pagination, db).await?;
    Ok(Json(cultivars))
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
        .await?
        .map_or_else(
            || Err(EndpointRejection::NotFound("Cultivar not found.".into())),
            |cultivar| Ok(Json(cultivar)),
        )
}

/// Handles the `POST /cultivars` route.
#[tracing::instrument(skip(db, form))]
pub async fn cultivar_create(
    _: AdminUser,
    State(db): State<DatabaseConnection>,
    form: CultivarCreateForm,
) -> EndpointResult<StatusCode> {
    Cultivar::insert(form.into(), db).await?;
    Ok(StatusCode::CREATED)
}

/// Handles the `PUT /cultivars/:cultivar_id` route.
#[tracing::instrument(skip(db, form))]
pub async fn cultivar_update(
    _: AdminUser,
    cultivar_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: CultivarUpdateForm,
) -> EndpointResult<StatusCode> {
    Cultivar::update(cultivar_id, form.into(), db).await?;
    Ok(StatusCode::OK)
}

/// Handles the `DELETE /cultivars/:cultivar_id` route.
#[tracing::instrument(skip(db,))]
pub async fn cultivar_delete(
    _: AdminUser,
    cultivar_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    Cultivar::delete(cultivar_id, db).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Handles the `GET /cultivars/index` route.
#[tracing::instrument(skip(db))]
pub async fn cultivar_index(
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<ModelIndex>> {
    let cultivar_index = Cultivar::index(db).await?;
    Ok(Json(cultivar_index))
}

/// Handles the `POST /cultivars/:cultivar_id/photo` route.
#[tracing::instrument(skip(db, multipart))]
pub async fn cultivar_image_upload(
    _: AdminUser,
    cultivar_id: ModelID,
    State(db): State<DatabaseConnection>,
    multipart: Multipart,
) -> EndpointResult<Json<String>> {
    let (handler, mut uploads) = files::accept_uploads(multipart, crate::CULTIVAR_MAX_IMAGE);
    handler.accept().await?; // Receive file from the client
    if let Some(file) = uploads.files().await {
        // Save an image to the file system
        let paths = file.save_image(CULTIVAR_UPLOAD_DIR).await?;

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
#[tracing::instrument(skip(db,))]
pub async fn cultivar_image_delete(
    _: AdminUser,
    cultivar_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    Cultivar::delete_photo(cultivar_id, db).await?;
    Ok(StatusCode::NO_CONTENT)
}
