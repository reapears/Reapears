//! Farm http handlers impls

use axum::{
    extract::{Json, Multipart, Query, State},
    http::StatusCode,
};

use crate::{
    auth::{AdminUser, CurrentUser, FarmerUser},
    endpoint::{EndpointRejection, EndpointResult},
    files,
    server::state::DatabaseConnection,
    settings::FARM_LOGO_UPLOAD_DIR,
    types::ModelID,
    types::{ModelIndex, Pagination},
};

use super::{
    forms::{FarmCreateForm, FarmUpdateForm},
    models::{Farm, FarmList},
    permissions::check_user_owns_farm,
    utils::delete_farm_logo,
};

/// Handles the `GET /farms` route.
#[tracing::instrument(skip(db))]
pub async fn farm_list(
    #[allow(unused_variables)] user: AdminUser,
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<FarmList>> {
    let pagination = pg.unwrap_or_default().0;
    Farm::records(pagination, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |farms| Ok(Json(farms)),
    )
}

/// Handles the `GET /farms/:farm_id` route.
#[tracing::instrument(skip(db))]
pub async fn farm_detail(
    farm_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<Farm>> {
    Farm::find(farm_id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |farm| {
            farm.map_or_else(
                || Err(EndpointRejection::NotFound("Farm not found".into())),
                |farm| Ok(Json(farm)),
            )
        },
    )
}

/// Handles the `POST /farms` route.
#[tracing::instrument(skip(user, db, form))]
pub async fn farm_create(
    user: CurrentUser,
    State(db): State<DatabaseConnection>,
    form: FarmCreateForm,
) -> EndpointResult<StatusCode> {
    Farm::insert(form.data(user.id), db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |_farm_id| Ok(StatusCode::CREATED),
    )
}

/// Handles the `PUT /farms/:farm_id` route.
#[tracing::instrument(skip(user, db, form))]
pub async fn farm_update(
    #[allow(unused_variables)] user: FarmerUser,
    farm_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: FarmUpdateForm,
) -> EndpointResult<StatusCode> {
    Farm::update(farm_id, form.into(), db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |_| Ok(StatusCode::OK),
    )
}

/// Handles the `DELETE /farms/:farm_id` route.
#[tracing::instrument(skip(user, db))]
pub async fn farm_delete(
    user: FarmerUser,
    farm_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    let check_permissions = check_user_owns_farm(user.0.id, farm_id, db.clone());
    match check_permissions.await {
        Ok(()) => Farm::delete(farm_id, db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::NO_CONTENT),
        ),
        Err(err) => Err(err),
    }
}

/// Handles the `GET /farms/:farm_id/locations` route.
#[tracing::instrument(skip(db))]
pub async fn farm_location_index(
    farm_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<ModelIndex>> {
    Farm::location_index(farm_id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |location_index| Ok(Json(location_index)),
    )
}

/// Handles the `POST /farms/:farm_id/logo` route.
#[tracing::instrument(skip(db))]
pub async fn farm_logo_upload(
    user: FarmerUser,
    farm_id: ModelID,
    State(db): State<DatabaseConnection>,
    multipart: Multipart,
) -> EndpointResult<Json<String>> {
    let check_permissions = check_user_owns_farm(user.0.id, farm_id, db.clone());

    match check_permissions.await {
        Ok(()) => {
            let (handler, mut uploads) = files::accept_uploads(multipart, 1);
            handler.accept().await?; // Receive logo from the client
            if let Some(file) = uploads.files().await {
                // Save a logo to the file system
                let saved_to = files::save_image(file, FARM_LOGO_UPLOAD_DIR).await?;

                // Save image path to the database
                let (new_logo, old_logo) =
                    Farm::insert_or_delete_logo(farm_id, Some(saved_to), db).await?;

                // Delete old logo
                if let Some(old_logo) = old_logo {
                    tokio::spawn(async move { delete_farm_logo(&old_logo).await });
                }

                Ok(Json(new_logo.unwrap()))
            } else {
                Err(EndpointRejection::BadRequest(
                    "Farm logo not received".into(),
                ))
            }
        }
        Err(err) => Err(err),
    }
}

/// Handles the `DELETE /farms/:farm_id/logo` route.
#[tracing::instrument(skip(db))]
pub async fn farm_logo_delete(
    user: FarmerUser,
    farm_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    let check_permissions = check_user_owns_farm(user.0.id, farm_id, db.clone());

    match check_permissions.await {
        Ok(()) => {
            // Delete the logo from the database
            let (_, old_logo) = Farm::insert_or_delete_logo(farm_id, None, db).await?;

            // Delete old logo
            if let Some(old_logo) = old_logo {
                tokio::spawn(async move { delete_farm_logo(&old_logo).await });
            }

            Ok(StatusCode::NO_CONTENT)
        }
        Err(err) => Err(err),
    }
}
