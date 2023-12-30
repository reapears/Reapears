//! Api key http handlers impls

use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use crate::{
    auth::{AdminUser, CurrentUser, SuperUser},
    endpoint::{EndpointRejection, EndpointResult},
    server::state::DatabaseConnection,
    types::ModelID,
};

use super::{ApiToken, ApiTokenForAppForm, ApiTokenList};

/// Handles the `GET /auth/api_key` route.
#[tracing::instrument(skip(db))]
pub async fn api_key_list(
    _: AdminUser,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<ApiTokenList>> {
    ApiToken::records(db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |tokens| Ok(Json(tokens)),
    )
}

/// Handles the  `GET /auth/api_key` route.
#[tracing::instrument(skip(db,))]
pub async fn api_key_delete(
    _: SuperUser,
    token_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    ApiToken::delete_by_id(token_id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |()| Ok(StatusCode::NO_CONTENT),
    )
}

/// Handles the `GET /auth/api_key/user` route.
#[tracing::instrument(skip(db))]
pub async fn generate_api_key_for_user(
    user: CurrentUser,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<String> {
    let (api_token, plaintext) = ApiToken::new_for_user(user.id);
    api_token.insert(db).await?;
    Ok(plaintext)
}

/// Handles the `GET /auth/api_key/app` route.
#[tracing::instrument(skip(db, form))]
pub async fn generate_api_key_for_app(
    _: SuperUser,
    State(db): State<DatabaseConnection>,
    form: ApiTokenForAppForm,
) -> EndpointResult<String> {
    let (api_token, plaintext) = ApiToken::new_for_app(form.name);
    api_token.insert(db).await?;
    Ok(plaintext)
}
