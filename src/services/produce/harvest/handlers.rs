//! Cultivar http handlers impls

use std::collections::HashMap;

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
    services::farmers::location::permissions::check_user_owns_location,
    settings::harvest_uploads_dir,
    types::Pagination,
};

use super::{
    forms::{HarvestCreateForm, HarvestInsertData, HarvestUpdateData, HarvestUpdateForm},
    models::{Harvest, HarvestImages, HarvestList},
    permissions::{check_user_can_update_harvest, check_user_owns_harvest},
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
    ModelId(harvest_id): ModelId<Uuid>,
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
#[tracing::instrument(skip(current_user, db, form))]
pub async fn harvest_create(
    current_user: CurrentUser,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<HarvestCreateForm>,
) -> EndpointResult<StatusCode> {
    if current_user.is_farmer {
        let values: HarvestInsertData = form.into();
        let check_permissions =
            check_user_owns_location(current_user.id, values.location_id, db.clone());
        match check_permissions.await {
            Ok(()) => Harvest::insert(values, db).await.map_or_else(
                |_err| Err(EndpointRejection::internal_server_error()),
                |_harvest_id| Ok(StatusCode::CREATED),
            ),
            Err(err) => Err(err),
        }
    } else {
        Err(EndpointRejection::BadRequest(
            "Create a farm, to start listing harvests!".into(),
        ))
    }
}

/// Handles the `PUT /harvests/:harvest_id` route.
#[tracing::instrument(skip(current_user, db, form))]
pub async fn harvest_update(
    current_user: CurrentUser,
    ModelId(harvest_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<HarvestUpdateForm>,
) -> EndpointResult<StatusCode> {
    let values: HarvestUpdateData = form.into();
    let check_permissions = if let Some(location_id) = values.location_id {
        // location id is provided, check also the location belongs to current user
        check_user_can_update_harvest(current_user.id, location_id, harvest_id, db.clone()).await
    } else {
        // Check only the harvest belong to the current user
        check_user_owns_harvest(current_user.id, harvest_id, db.clone()).await
    };
    match check_permissions {
        Ok(()) => Harvest::update(harvest_id, values, db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::OK),
        ),

        Err(err) => Err(err),
    }
}

/// Handles the `DELETE /harvests/:harvest_id` route.
#[tracing::instrument(skip(current_user, db))]
pub async fn harvest_delete(
    current_user: CurrentUser,
    ModelId(harvest_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    let check_permissions = check_user_owns_harvest(current_user.id, harvest_id, db.clone());
    match check_permissions.await {
        Ok(()) => Harvest::delete(harvest_id, db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::NO_CONTENT),
        ),
        Err(err) => Err(err),
    }
}

/// Handles the `POST /harvests/:harvest_id/photos` route.
#[tracing::instrument(skip(current_user, db, multipart))]
#[allow(clippy::redundant_closure)]
pub async fn harvest_image_uploads(
    current_user: CurrentUser,
    ModelId(harvest_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
    multipart: Multipart,
) -> EndpointResult<Json<HarvestImages>> {
    let check_permissions = check_user_owns_harvest(current_user.id, harvest_id, db.clone());

    match check_permissions.await {
        Ok(()) => {
            let (handler, mut uploads) = files::accept_uploads(multipart, HARVEST_MAX_IMAGE);
            handler.accept().await?; // Receive files from the client

            let mut file_names = HashMap::new();
            let mut field_names = ["cover", "n1", "n2", "n3", "n4"].into_iter();

            while let Some(file) = uploads.files().await {
                let label = field_names
                    .next()
                    .ok_or_else(|| EndpointRejection::internal_server_error())?;

                file_names.insert(label, format!("{}.jpg", file.id));

                // Save an image to the file system
                files::save_image(file, harvest_uploads_dir()).await?;
            }

            // Convert to HarvestImages
            // Safety: We're checking above that file.field_name is in field_names
            let json = serde_json::to_value(file_names).unwrap();
            let saved_to: HarvestImages = serde_json::from_str(&json.to_string()).unwrap();

            // Save image path to the database
            let (new_images, old_images) = Harvest::insert_photos(harvest_id, saved_to, db).await?;

            // Delete old images
            if let Some(old_images) = old_images {
                tokio::spawn(async move { delete_harvest_photos(old_images).await });
            }

            Ok(Json(new_images))
        }
        Err(err) => Err(err),
    }
}

/// Handles the `DELETE /harvests/:harvest_id/photos` route.
///
/// Deletes all images uploaded for this harvest
#[tracing::instrument(skip(current_user, db))]
pub async fn harvest_image_delete(
    current_user: CurrentUser,
    ModelId(harvest_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    let check_permissions = check_user_owns_harvest(current_user.id, harvest_id, db.clone());
    match check_permissions.await {
        Ok(()) => Harvest::delete_photos(harvest_id, db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::NO_CONTENT),
        ),
        Err(err) => Err(err),
    }
}
