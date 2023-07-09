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
    permissions::check_user_owns_harvest,
    utils::delete_harvest_photos,
    HARVEST_MAX_IMAGE,
};

/// Handles the `GET /harvests` route.
#[tracing::instrument(skip(db))]
pub async fn harvest_list(
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<HarvestList>> {
    let pagination = pg.unwrap_or_default().0;
    Harvest::records(pagination, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |harvest| Ok(Json(harvest)),
    )
}

/// Handles the `GET /harvests/:harvest_id` route.
#[tracing::instrument(skip(db))]
pub async fn harvest_detail(
    harvest_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<Harvest>> {
    Harvest::find(harvest_id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |harvest| {
            harvest.map_or_else(
                || Err(EndpointRejection::NotFound("Harvest not found.".into())),
                |harvest| Ok(Json(harvest)),
            )
        },
    )
}

/// Handles the `POST /harvests` route.
#[tracing::instrument(skip(user, db, form))]
#[allow(unused_variables)]
pub async fn harvest_create(
    #[allow(unused_variables)] user: FarmerUser,
    State(db): State<DatabaseConnection>,
    form: HarvestCreateForm,
) -> EndpointResult<StatusCode> {
    Harvest::insert(form.into(), db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |_harvest_id| Ok(StatusCode::CREATED),
    )
}

/// Handles the `PUT /harvests/:harvest_id` route.
#[tracing::instrument(skip(user, db, form))]
pub async fn harvest_update(
    #[allow(unused_variables)] user: FarmerUser,
    harvest_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: HarvestUpdateForm,
) -> EndpointResult<StatusCode> {
    Harvest::update(harvest_id, form.into(), db)
        .await
        .map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::OK),
        )
}

/// Handles the `DELETE /harvests/:harvest_id` route.
#[tracing::instrument(skip(user, db))]
pub async fn harvest_delete(
    user: FarmerUser,
    harvest_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    let check_permissions = check_user_owns_harvest(user.id(), harvest_id, db.clone());
    match check_permissions.await {
        Ok(()) => Harvest::delete(harvest_id, db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::NO_CONTENT),
        ),
        Err(err) => Err(err),
    }
}

/// Handles the `POST /harvests/:harvest_id/photos` route.
#[tracing::instrument(skip(user, db, multipart))]
#[allow(clippy::redundant_closure)]
pub async fn harvest_image_uploads(
    user: FarmerUser,
    harvest_id: ModelID,
    State(db): State<DatabaseConnection>,
    multipart: Multipart,
) -> EndpointResult<Json<Vec<String>>> {
    let check_permissions = check_user_owns_harvest(user.id(), harvest_id, db.clone());

    match check_permissions.await {
        Ok(()) => {
            let (handler, mut uploads) = files::accept_uploads(multipart, HARVEST_MAX_IMAGE);

            // spawn image receiving task
            tokio::spawn(async move { handler.accept().await });

            let mut paths = Vec::with_capacity(HARVEST_MAX_IMAGE as usize);

            while let Some(file) = uploads.files().await {
                // Save an image to the file system
                paths.push(format!("{}.jpg", file.id));
                files::save_image(file, HARVEST_UPLOAD_DIR).await?;
            }

            // Save image path to the database
            // and delete old images if there is some
            if let Some(old_images) = Harvest::insert_photos(harvest_id, paths.clone(), db).await? {
                tokio::spawn(async move { delete_harvest_photos(old_images).await });
            }

            Ok(Json(paths))
        }
        Err(err) => Err(err),
    }
}

/// Handles the `DELETE /harvests/:harvest_id/photos` route.
///
/// Deletes all images uploaded for this harvest
#[tracing::instrument(skip(user, db))]
pub async fn harvest_image_delete(
    user: FarmerUser,
    harvest_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    let check_permissions = check_user_owns_harvest(user.0.id, harvest_id, db.clone());
    match check_permissions.await {
        Ok(()) => Harvest::delete_photos(harvest_id, db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::NO_CONTENT),
        ),
        Err(err) => Err(err),
    }
}
