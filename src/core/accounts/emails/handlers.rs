//! Email http handlers impls

use axum::{extract::State, http::StatusCode};
use axum_extra::extract::PrivateCookieJar;

use crate::{
    accounts::passwords::{get_password_verified, remove_password_verified_cookie},
    auth::{hash_token, CurrentUser},
    endpoint::{EndpointRejection, EndpointResult, ValidatedJson},
    mail::{emails::confirmation_code_email, Mail},
    server::state::DatabaseConnection,
};

use super::{
    forms::{CodeConfirmForm, EmailForm},
    EmailModel,
};

/// Handles the `POST /account/email-exists` route.
///
/// Check by email if the user account exists
#[tracing::instrument(skip(db))]
pub async fn email_exists(
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<EmailForm>,
) -> EndpointResult<StatusCode> {
    if EmailModel::exists_and_verified(form.email, db).await? {
        Ok(StatusCode::OK)
    } else {
        Err(EndpointRejection::BadRequest("Account not found".into()))
    }
}

/// Handles the `POST /account/settings/change-email/` route.
#[tracing::instrument(skip(db, cookie_jar, current_user, form))]
pub async fn email_update(
    current_user: CurrentUser,
    cookie_jar: PrivateCookieJar,
    State(db): State<DatabaseConnection>,
    State(outlook): State<Mail>,
    ValidatedJson(form): ValidatedJson<EmailForm>,
) -> EndpointResult<(PrivateCookieJar, &'static str)> {
    // should check the email does not exist already
    // If the password is not verified, don't permit email update
    if get_password_verified(&cookie_jar).is_none() {
        return Err(EndpointRejection::unauthorized());
    }

    let user_id = current_user.id;

    let (values, code) = form.pending_update_data();
    EmailModel::insert_pending_update(user_id, values, db.clone()).await?;

    // Send confirmation code to an existing email
    let (first_name, email_address) = EmailModel::find_user(user_id, db).await?;

    let subject = "Reapears email change verification.";
    let email = confirmation_code_email(&first_name, &email_address, subject, &code)?;
    outlook.send(email).await?;

    Ok((cookie_jar, "Confirmation code was sent to your email"))
}

/// Handles the `POST /account/settings/confirm-email` route.
#[tracing::instrument(skip(db, current_user, form))]
pub async fn email_verify(
    current_user: CurrentUser,
    cookie_jar: PrivateCookieJar,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<CodeConfirmForm>,
) -> EndpointResult<(StatusCode, PrivateCookieJar)> {
    // If the password is not verified, don't permit email update
    if get_password_verified(&cookie_jar).is_none() {
        return Err(EndpointRejection::unauthorized());
    }

    let user_id = current_user.id;

    // Verify email pending update
    let code = hash_token(form.code.as_bytes());
    if EmailModel::pending_update_exists(user_id, code, db.clone()).await? {
        EmailModel::update(user_id, db).await?;
        let cookie_jar = remove_password_verified_cookie(cookie_jar);
        Ok((StatusCode::OK, cookie_jar))
    } else {
        Err(EndpointRejection::BadRequest(
            "Your verification code is incorrect".into(),
        ))
    }
}
