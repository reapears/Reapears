//! Session http handlers impls

use axum::extract::{Query, State};
use axum_extra::{extract::PrivateCookieJar, headers::UserAgent, TypedHeader};

use crate::{
    auth::{get_current_user, CurrentUser},
    endpoint::EndpointResult,
    server::state::DatabaseConnection,
};

use super::{
    add_session_cookie,
    forms::{LoginForm, SuccessRedirect},
    get_session_token_hash,
    models::Session,
    remove_session_cookie,
};

/// Handles the `POST /account/login` route.
#[tracing::instrument(skip(db, form, cookie_jar))]
pub async fn login(
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    redirect_to: Option<Query<SuccessRedirect>>,
    cookie_jar: PrivateCookieJar,
    State(db): State<DatabaseConnection>,
    form: LoginForm,
) -> EndpointResult<(PrivateCookieJar, String)> {
    // Get success redirect if provided
    let redirect = redirect_to.unwrap_or_default();
    let return_to = redirect.0.return_to;

    // Verify the user is not logged-in already
    // so we don't insert duplicate sessions in the database
    if let Some(token) = get_session_token_hash(&cookie_jar) {
        if (get_current_user(token, db.clone()).await?).is_some() {
            return Ok((cookie_jar, return_to));
        }
    }

    // Login-user
    let user_agent = user_agent.to_string().to_lowercase();
    let (values, token) = form.session_data(user_agent);
    Session::insert(values, db).await?;
    let cookie_jar = add_session_cookie(cookie_jar, token);

    Ok((cookie_jar, return_to))
}

/// Handles the `POST /account/logout` route.
#[tracing::instrument(skip(db, current_user, cookie_jar))]
#[allow(unused_variables)]
pub async fn logout(
    current_user: CurrentUser,
    cookie_jar: PrivateCookieJar,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<PrivateCookieJar> {
    // Safety: authorization passed so the token is there
    let token_hash = get_session_token_hash(&cookie_jar).unwrap();
    Session::delete(token_hash, db).await?;
    let cookie_jar = remove_session_cookie(cookie_jar);
    Ok(cookie_jar)
}
