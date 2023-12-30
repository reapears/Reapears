//! Password http handlers impls

use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use axum_extra::extract::PrivateCookieJar;

use crate::{
    accounts::user::models::User,
    auth::{hash_token, CurrentUser, Token, TokenConfirm},
    endpoint::{EndpointRejection, EndpointResult},
    mail::Mail,
    server::state::DatabaseConnection,
    SERVER_DOMAIN_NAME,
};

use super::{
    add_password_verified_cookie, check_password,
    forms::{PasswordChangeForm, PasswordForgotForm, PasswordResetForm, PasswordVerifyForm},
    password_reset_token_expired, PasswordModel,
};

/// Handles the `POST /account/settings/verify-password` route.
///
/// Authorizes logged-in user to perform
/// sensitive tasks such as, changing email.
#[tracing::instrument(skip(db, form))]
pub async fn password_verify(
    current_user: CurrentUser,
    cookie_jar: PrivateCookieJar,
    State(db): State<DatabaseConnection>,
    form: PasswordVerifyForm,
) -> EndpointResult<(PrivateCookieJar, StatusCode)> {
    check_password(current_user.id, form.password, db).await?;
    let cookie_jar = add_password_verified_cookie(cookie_jar);
    Ok((cookie_jar, StatusCode::OK))
}

/// Handles the `POST /account/settings/change-password` route.
///
/// Changes user password
#[tracing::instrument(skip(db, form))]
pub async fn password_change(
    current_user: CurrentUser,
    State(db): State<DatabaseConnection>,
    form: PasswordChangeForm,
) -> EndpointResult<StatusCode> {
    let password_hash = form.try_phc().await?;
    PasswordModel::update(current_user.id, password_hash, db).await?;
    Ok(StatusCode::OK)
}

/// Handles the `POST /account/reset-password` route.
///
/// Used for when a user want to change their forgotten password
#[tracing::instrument(skip(db))]
pub async fn password_reset(
    confirm_token: Option<Query<TokenConfirm>>,
    State(db): State<DatabaseConnection>,
    form: PasswordResetForm,
) -> EndpointResult<&'static str> {
    static ERR_MSG: &str = "Your password rest link is no longer valid.";

    let Some(Query(confirm_token)) = confirm_token else {
        return Err(EndpointRejection::BadRequest(
            "Password reset token required!".into(),
        ));
    };

    let token = confirm_token.token;
    // Verify token
    let token_hash = hash_token(token.as_bytes());
    let Some((user_id, token_created_at)) =
        PasswordModel::find_token(token_hash, db.clone()).await?
    else {
        return Err(EndpointRejection::BadRequest(ERR_MSG.into()));
    };
    if password_reset_token_expired(token_created_at) {
        return Err(EndpointRejection::BadRequest(ERR_MSG.into()));
    }

    // Update password
    let phc_string = form.try_phc().await?;
    PasswordModel::update(user_id, phc_string, db).await?;

    Ok("Your password has been reset successfully")
}

/// Handles the `POST /account/password-forgot` route.
#[tracing::instrument(skip(db, form))]
pub async fn password_forgot(
    State(db): State<DatabaseConnection>,
    State(outlook): State<Mail>,
    form: PasswordForgotForm,
) -> EndpointResult<&'static str> {
    let (plaintext, hash) = Token::default().into_parts();
    let email_address = form.email;
    let Some((user_id, first_name)) =
        User::find_by_email(email_address.clone(), db.clone()).await?
    else {
        return Err(EndpointRejection::BadRequest(
            "Sorry, we could not find your account.".into(),
        ));
    };
    PasswordModel::insert_token(user_id, hash, db).await?;

    // Send password reset email
    let domain = SERVER_DOMAIN_NAME.get().unwrap();
    let link = format!("{domain}/account/reset-password?token={plaintext}");
    let email = outlook.password_reset(&first_name, &email_address, &link)?;
    outlook.send(email).await?;

    Ok("Your password reset link was sent to your email ")
}
