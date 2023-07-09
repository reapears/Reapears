//! Email http handlers impls

use axum::{extract::State, http::StatusCode};
use axum_extra::extract::PrivateCookieJar;

use crate::{
    accounts::passwords::{get_password_verified, remove_password_verified_cookie},
    auth::{hash_token, CurrentUser, Token},
    endpoint::{EndpointRejection, EndpointResult},
    mail::Mail,
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
    form: EmailForm,
) -> EndpointResult<StatusCode> {
    if EmailModel::exists_and_verified(form.email, db).await? {
        Ok(StatusCode::OK)
    } else {
        Err(EndpointRejection::BadRequest("Account not found".into()))
    }
}

/// Handles the `POST /account/settings/change-email/` route.
#[tracing::instrument(skip(db, cookie_jar, user, form))]
pub async fn email_update(
    user: CurrentUser,
    cookie_jar: PrivateCookieJar,
    State(db): State<DatabaseConnection>,
    State(outlook): State<Mail>,
    form: EmailForm,
) -> EndpointResult<(PrivateCookieJar, &'static str)> {
    // should check the email does not exist already
    // If the password is not verified, don't permit email update
    if get_password_verified(&cookie_jar).is_none() {
        return Err(EndpointRejection::unauthorized());
    }

    let (values, code) = form.pending_update_data();
    let new_email = values.new_email.clone();
    EmailModel::insert_pending_update(user.id, values, db.clone()).await?;

    // Send confirmation code to an existing email
    let (first_name, email_address) = EmailModel::find_user(user.id, db).await?;

    let email = outlook.approve_email_change(&first_name, &email_address, &new_email, &code)?;
    outlook.send(email).await?;

    Ok((
        remove_password_verified_cookie(cookie_jar),
        "Approve this email change by entering the code we just you at your current email",
    ))
}

/// Handles the `POST /account/settings/approve-email-change` route.
#[tracing::instrument(skip(db, user, form))]
pub async fn email_change_approve(
    user: CurrentUser,
    State(db): State<DatabaseConnection>,
    State(outlook): State<Mail>,
    form: CodeConfirmForm,
) -> EndpointResult<(StatusCode, &'static str)> {
    // Verify email pending update
    let approval_code = hash_token(form.code.as_bytes());
    if let Some(new_email) = EmailModel::approve_pending_update(approval_code, db.clone()).await? {
        // Send email
        let (code, hash) = Token::new_code().into_parts();

        EmailModel::insert_new_email_verify_code(user.id, hash, db.clone()).await?;

        let (first_name, _) = EmailModel::find_user(user.id, db).await?;

        let email = outlook.verify_new_email(&first_name, &new_email, &code)?;
        outlook.send(email).await?;

        Ok((
            StatusCode::OK,
            "Verify your new email to by entering the code we just sent you to complete",
        ))
    } else {
        EmailModel::delete_pending_updates(user.id, db).await?;
        Err(EndpointRejection::BadRequest(
            "Your verification code is incorrect".into(),
        ))
    }
}

/// Handles the `POST /account/settings/confirm-new-email` route.
#[tracing::instrument(skip(db, user, form))]
pub async fn new_email_change_verify(
    user: CurrentUser,
    State(db): State<DatabaseConnection>,
    form: CodeConfirmForm,
) -> EndpointResult<(StatusCode, &'static str)> {
    // Verify email pending update
    let code = hash_token(form.code.as_bytes());
    if EmailModel::verify_pending_update(code, db.clone()).await? {
        EmailModel::update(user.id, db).await?;
        Ok((StatusCode::OK, "Your email was changed successfully"))
    } else {
        EmailModel::delete_pending_updates(user.id, db).await?;
        Err(EndpointRejection::BadRequest(
            "Your verification code is incorrect".into(),
        ))
    }
}
