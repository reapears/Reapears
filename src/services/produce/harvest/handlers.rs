//! Cultivar http handlers impls

use axum::{
    extract::{Json, Multipart, Query, State},
    http::StatusCode,
};

use crate::{
    auth::FarmerUser,
    endpoint::{EndpointRejection, EndpointResult},
    files,
    server::state::DatabaseConnection,
    settings::HARVEST_UPLOAD_DIR,
    types::{ModelID, Pagination},
};

use super::{
    forms::{HarvestCreateForm, HarvestUpdateForm},
    models::{Harvest, HarvestList},
    permissions::HarvestOwnershipPermission,
    utils::delete_harvest_photos,
};

/// Handles the `GET /harvests` route.
#[tracing::instrument(skip(db))]
pub async fn harvest_list(
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<HarvestList>> {
    let pagination = pg.unwrap_or_default().0;
    let harvests = Harvest::records(pagination, db).await?;
    Ok(Json(harvests))
}

/// Handles the `GET /harvests/:harvest_id` route.
#[tracing::instrument(skip(db))]
pub async fn harvest_detail(
    harvest_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<Harvest>> {
    Harvest::find(harvest_id, db).await?.map_or_else(
        || Err(EndpointRejection::NotFound("Harvest not found.".into())),
        |harvest| Ok(Json(harvest)),
    )
}

/// Handles the `POST /harvests` route.
#[tracing::instrument(skip(db, form))]
pub async fn harvest_create(
    _: FarmerUser,
    State(db): State<DatabaseConnection>,
    form: HarvestCreateForm,
) -> EndpointResult<StatusCode> {
    Harvest::insert(form.into(), db).await?;
    Ok(StatusCode::CREATED)
}

/// Handles the `PUT /harvests/:harvest_id` route.
#[tracing::instrument(skip(db, form))]
pub async fn harvest_update(
    _: FarmerUser,
    harvest_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: HarvestUpdateForm,
) -> EndpointResult<StatusCode> {
    Harvest::update(harvest_id, form.into(), db).await?;
    Ok(StatusCode::OK)
}

/// Handles the `DELETE /harvests/:harvest_id` route.
#[tracing::instrument(skip(db))]
pub async fn harvest_delete(
    _: HarvestOwnershipPermission,
    harvest_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    Harvest::delete(harvest_id, db).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Handles the `POST /harvests/:harvest_id/photos` route.
#[tracing::instrument(skip(db, multipart))]
#[allow(clippy::redundant_closure)]
pub async fn harvest_image_uploads(
    _: HarvestOwnershipPermission,
    harvest_id: ModelID,
    State(db): State<DatabaseConnection>,
    multipart: Multipart,
) -> EndpointResult<Json<Vec<String>>> {
    let (handler, mut uploads) = files::accept_uploads(multipart, crate::HARVEST_MAX_IMAGE);

    // Receive images
    tokio::spawn(async move { handler.accept().await });

    let mut paths = Vec::with_capacity(crate::HARVEST_MAX_IMAGE as usize);
    while let Some(file) = uploads.files().await {
        // Save an image to the file system
        paths.push(format!("{}.jpg", file.id));
        file.save_image(HARVEST_UPLOAD_DIR).await?; // spawn ??
    }

    // Save image path to the database
    // and delete old images if there is some
    if let Some(old_images) = Harvest::insert_photos(harvest_id, paths.clone(), db).await? {
        tokio::spawn(async move { delete_harvest_photos(old_images.into_iter()).await });
    }

    Ok(Json(paths))
}

/// Handles the `DELETE /harvests/:harvest_id/photos` route.
///
/// Deletes all images uploaded for this harvest
#[tracing::instrument(skip(db))]
pub async fn harvest_image_delete(
    _: HarvestOwnershipPermission,
    harvest_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    Harvest::delete_photos(harvest_id, db).await?;
    Ok(StatusCode::NO_CONTENT)
}
