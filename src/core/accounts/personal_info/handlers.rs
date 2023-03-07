//! `PersonalInfo` http handlers impls

use axum::{extract::State, http::StatusCode, Json};

use crate::{
    auth::CurrentUser,
    endpoint::{EndpointRejection, EndpointResult, ValidatedJson},
    server::state::DatabaseConnection,
};

use super::{forms::PersonalInfoUpdateForm, models::PersonalInfo};

/// Handles the `GET /account/settings/personal-info` route.
#[tracing::instrument(skip(db))]
pub async fn user_personal_info(
    current_user: CurrentUser,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<PersonalInfo>> {
    PersonalInfo::find(current_user.id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |personal_info| {
            personal_info.map_or_else(
                || {
                    Err(EndpointRejection::NotFound(
                        "User personal infos not found".into(),
                    ))
                },
                |personal_info| Ok(Json(personal_info)),
            )
        },
    )
}

/// Handles the `PUT /account/settings/personal-info` route.
#[tracing::instrument(skip(db))]
pub async fn user_personal_info_update(
    current_user: CurrentUser,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<PersonalInfoUpdateForm>,
) -> EndpointResult<StatusCode> {
    PersonalInfo::update(current_user.id, form.into(), db)
        .await
        .map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::OK),
        )
}
