//! User http handlers impls

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    accounts::emails::EmailModel,
    auth::{hash_token, AdminUser, CurrentUser, SuperUser, Token, TokenConfirm},
    endpoint::{EndpointRejection, EndpointResult},
    mail::Mail,
    server::state::DatabaseConnection,
    types::Pagination,
    SERVER_DOMAIN_NAME,
};

use super::{
    account_confirm_expiry_time,
    forms::{AccountLockForm, SignUpForm, UserIdForm},
    models::{User, UserList},
};

/// Handles the `GET /account/users` route.
#[tracing::instrument(skip(db))]
pub async fn user_list(
    user: AdminUser,
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<UserList>> {
    let pagination = pg.unwrap_or_default().0;
    let users = User::records(pagination, db).await?;
    Ok(Json(users))
}

/// Handles the `POST /account/signup` route.
#[tracing::instrument(skip(db))]
pub async fn signup(
    State(db): State<DatabaseConnection>,
    State(outlook): State<Mail>,
    form: SignUpForm,
) -> EndpointResult<&'static str> {
    let (plaintext, hash) = Token::default().into_parts();
    let values = form.try_data(hash).await?;
    let first_name = values.first_name.clone();
    let email_address = values.email.email.clone();

    User::insert(values, db).await?;

    // Send confirmation email
    let domain = SERVER_DOMAIN_NAME.get().unwrap();
    let link = format!("{domain}/account/confirm?token={plaintext}");
    let email = outlook.account_confirm(&first_name, &email_address, &link)?;
    outlook.send(email).await?;

    Ok("Please confirm your email address by clicking the email we just sent you.")
}

/// Handles the `POST account/confirm` route.
///
/// Confirms user email address on account registration
#[tracing::instrument(skip(confirm_token, db))]
pub async fn account_confirm(
    confirm_token: Option<Query<TokenConfirm>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<&'static str> {
    let Some(Query(confirm_token)) = confirm_token else {
        return Err(EndpointRejection::BadRequest(
            "Confirmation token required!".into(),
        ));
    };

    let token = hash_token(confirm_token.token.as_bytes());

    let Some((user_id, email, Some(token_generated_at))) =
        EmailModel::find_by_token(token, db.clone()).await?
    else {
        return Err(EndpointRejection::BadRequest(
            "Your confirmation link is no longer valid. \
Your account may already be activated or may have cancelled your registration."
                .into(),
        ));
    };

    // Verify token has not expired
    let expiry_time = account_confirm_expiry_time();
    if token_generated_at < expiry_time {
        User::delete_unverified(user_id, db).await?;
        return Err(EndpointRejection::BadRequest(
            "Your confirmation link is no longer valid. Please SignUp again.".into(),
        ));
    }

    EmailModel::verify(email, db).await?;

    Ok("Your account has been verified")
}

/// Handles the `POST /account/lock` route.
///
/// Locks the user account, the user will not be able to login
///  until the account is unlocked.
#[tracing::instrument(skip(db))]
pub async fn account_lock(
    user: SuperUser,
    State(db): State<DatabaseConnection>,
    form: AccountLockForm,
) -> EndpointResult<StatusCode> {
    User::lock_account(form.into(), db).await?;
    Ok(StatusCode::OK)
}

/// Handles the `POST /account/unlock` route.
#[tracing::instrument(skip(db))]
pub async fn account_unlock(
    user: SuperUser,
    State(db): State<DatabaseConnection>,
    form: UserIdForm,
) -> EndpointResult<StatusCode> {
    User::unlock_account(form.user_id, db).await?;
    Ok(StatusCode::OK)
}

/// Handles the `DELETE /account/deactivate` route.
///
/// Permanently deletes the user from the platform
#[tracing::instrument(skip(user, db))]
pub async fn account_deactivate(
    user: CurrentUser,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    User::delete(user.id, db).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Handles the `POST /account/unlock` route.
#[tracing::instrument(skip(db))]
pub async fn user_make_superuser(
    _: SuperUser,
    State(db): State<DatabaseConnection>,
    form: UserIdForm,
) -> EndpointResult<StatusCode> {
    User::set_superuser(form.user_id, true, db).await?;
    Ok(StatusCode::OK)
}

/// Handles the `POST /account/unlock` route.
#[tracing::instrument(skip(db))]
pub async fn user_revoke_superuser(
    _: SuperUser,
    State(db): State<DatabaseConnection>,
    form: UserIdForm,
) -> EndpointResult<StatusCode> {
    User::set_superuser(form.user_id, false, db).await?;
    Ok(StatusCode::OK)
}

/// Handles the `POST /account/unlock` route.
#[tracing::instrument(skip(db))]
pub async fn user_make_staff(
    _: SuperUser,
    State(db): State<DatabaseConnection>,
    form: UserIdForm,
) -> EndpointResult<StatusCode> {
    User::set_staff(form.user_id, true, db).await?;
    Ok(StatusCode::OK)
}

/// Handles the `POST /account/unlock` route.
#[tracing::instrument(skip(db))]
pub async fn user_revoke_staff(
    _: SuperUser,
    State(db): State<DatabaseConnection>,
    form: UserIdForm,
) -> EndpointResult<StatusCode> {
    User::set_staff(form.user_id, false, db).await?;
    Ok(StatusCode::OK)
}
