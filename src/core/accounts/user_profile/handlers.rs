//! `UserProfile` http handlers impls

use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};

use crate::{
    auth::CurrentUser,
    endpoint::{EndpointRejection, EndpointResult},
    files,
    server::state::DatabaseConnection,
    settings::USER_UPLOAD_DIR,
    types::ModelID,
};

use super::{forms::UserProfileUpdateForm, models::UserProfile, utils::delete_user_photo};

/// Handles the `GET account/users/:id/profile` route.
#[tracing::instrument(skip(db))]
pub async fn user_profile(
    id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<UserProfile>> {
    UserProfile::find(id, db).await?.map_or_else(
        || Err(EndpointRejection::NotFound("User profile not found".into())),
        |profile| Ok(Json(profile)),
    )
}

/// Handlers the `GET /account/users/profile` route.
pub async fn user_my_profile(
    user: CurrentUser,
    db: State<DatabaseConnection>,
) -> EndpointResult<Json<UserProfile>> {
    user_profile(user.id, db).await
}

/// Handles the `PUT /account/users/profile` route.
#[tracing::instrument(skip(db))]
pub async fn user_profile_update(
    user: CurrentUser,
    State(db): State<DatabaseConnection>,
    form: UserProfileUpdateForm,
) -> EndpointResult<StatusCode> {
    UserProfile::create_or_update(user.id, form.into(), db).await?;
    Ok(StatusCode::OK)
}

/// Handles the `POST /account/users/profile/photo` route.
#[tracing::instrument(skip(db))]
pub async fn user_photo_upload(
    user: CurrentUser,
    State(db): State<DatabaseConnection>,
    multipart: Multipart,
) -> EndpointResult<Json<String>> {
    let (handler, mut uploads) = files::accept_uploads(multipart, crate::USER_MAX_PROFILE_PHOTO);
    handler.accept().await?; // Receive photo from the client
    if let Some(file) = uploads.files().await {
        // Save an image to the file system
        let saved_to = file.save_image(USER_UPLOAD_DIR).await?;

        // Save image path to the database
        let (new_photo, old_photo) = UserProfile::insert_photo(user.id, saved_to, db).await?;

        // Delete old photo
        if let Some(old_photo) = old_photo {
            tokio::spawn(async move { delete_user_photo(&old_photo).await });
        }

        Ok(Json(new_photo))
    } else {
        Err(EndpointRejection::BadRequest(
            "User profile photo not received".into(),
        ))
    }
}
