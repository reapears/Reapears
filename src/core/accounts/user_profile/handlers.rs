//! `UserProfile` http handlers impls

use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    auth::CurrentUser,
    endpoint::{EndpointRejection, EndpointResult, ModelId, ValidatedJson},
    files,
    server::state::DatabaseConnection,
    settings::user_uploads_dir,
};

use super::{
    forms::UserProfileUpdateForm, models::UserProfile, utils::delete_user_photo,
    USER_MAX_PROFILE_PHOTO,
};

/// Handles the `GET account/users/:id/profile` route.
#[tracing::instrument(skip(db))]
pub async fn user_profile(
    ModelId(id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<UserProfile>> {
    UserProfile::find(id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |profile| {
            profile.map_or_else(
                || Err(EndpointRejection::NotFound("User profile not found".into())),
                |profile| Ok(Json(profile)),
            )
        },
    )
}

/// Handlers the `GET /account/users/profile` route.
pub async fn user_my_profile(
    current_user: CurrentUser,
    db: State<DatabaseConnection>,
) -> EndpointResult<Json<UserProfile>> {
    user_profile(ModelId(current_user.id), db).await
}

/// Handles the `PUT /account/users/profile` route.
#[tracing::instrument(skip(db))]
pub async fn user_profile_update(
    current_user: CurrentUser,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<UserProfileUpdateForm>,
) -> EndpointResult<StatusCode> {
    let user_id = current_user.id;
    UserProfile::create_or_update(user_id, form.into(), db)
        .await
        .map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::OK),
        )
}

/// Handles the `POST /account/users/profile/photo` route.
#[tracing::instrument(skip(db))]
pub async fn user_photo_upload(
    current_user: CurrentUser,
    State(db): State<DatabaseConnection>,
    multipart: Multipart,
) -> EndpointResult<Json<String>> {
    let (handler, mut uploads) = files::accept_uploads(multipart, USER_MAX_PROFILE_PHOTO);
    handler.accept().await?; // Receive photo from the client
    if let Some(file) = uploads.files().await {
        // Save an image to the file system
        let saved_to = files::save_image(file, user_uploads_dir()).await?;

        // Save image path to the database
        let (new_photo, old_photo) =
            UserProfile::insert_photo(current_user.id, saved_to, db).await?;

        // Delete old photo
        if let Some(old_photo) = old_photo {
            tokio::spawn(async move { delete_user_photo(&old_photo).await });
        }

        Ok(Json(new_photo))
    } else {
        Err(EndpointRejection::BadRequest(
            "User profile photo is not received".into(),
        ))
    }
}