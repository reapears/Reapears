//! User http handlers impls

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    accounts::emails::EmailModel,
    auth::{hash_token, CurrentUser, Token, TokenConfirm},
    endpoint::{EndpointRejection, EndpointResult, ValidatedJson},
    mail::{emails::account_confirmation_email, Mail},
    server::state::DatabaseConnection,
    settings::SERVER_DOMAIN,
    types::Pagination,
};

use super::{
    account_confirm_expiry_time,
    forms::{AccountLockForm, AccountUnlockForm, SignUpForm},
    models::{User, UserList},
};

/// Handles the `GET /account/users` route.
#[tracing::instrument(skip(db))]
pub async fn user_list(
    // current_user: CurrentUser,
    pg: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<UserList>> {
    let pagination = pg.unwrap_or_default().0;
    User::records(pagination, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |users| Ok(Json(users)),
    )
}

/// Handles the `POST /account/signup` route.
#[tracing::instrument(skip(db))]
pub async fn signup(
    State(db): State<DatabaseConnection>,
    State(outlook): State<Mail>,
    ValidatedJson(form): ValidatedJson<SignUpForm>,
) -> EndpointResult<&'static str> {
    let (plaintext, hash) = Token::default().into_parts();
    let values = form.try_data(hash).await?;
    let first_name = values.first_name.clone();
    let email_address = values.email.email.clone();

    User::insert(values, db).await?;

    // Send confirmation email
    let link = format!("{SERVER_DOMAIN}/account/confirm?token={plaintext}");
    let subject = "[Reapears] Please verify your email address.";
    let email = account_confirmation_email(&first_name, &email_address, subject, &link)?;
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
    let Some(Query(confirm_token)) = confirm_token else{
        return Err(EndpointRejection::BadRequest("Confirmation token required!".into()));
    };

    let token = hash_token(confirm_token.token.as_bytes());

    let Some((user_id, email, Some(token_generated_at))) =
        EmailModel::find_by_token(token, db.clone()).await? else{
        return Err(EndpointRejection::BadRequest(
            "Your confirmation link is no longer valid. \
Your account may already be activated or may have cancelled your registration.".into(),
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
    current_user: CurrentUser,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<AccountLockForm>,
) -> EndpointResult<StatusCode> {
    if current_user.is_superuser {
        User::lock_account(form.into(), db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::OK),
        )
    } else {
        Err(EndpointRejection::forbidden())
    }
}

/// Handles the `POST /account/unlock` route.
#[tracing::instrument(skip(db))]
pub async fn account_unlock(
    current_user: CurrentUser,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<AccountUnlockForm>,
) -> EndpointResult<StatusCode> {
    if current_user.is_superuser {
        User::unlock_account(form.user_id, db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::OK),
        )
    } else {
        Err(EndpointRejection::forbidden())
    }
}

/// Handles the `DELETE /account/deactivate` route.
///
/// Permanently deletes the user from the platform
#[tracing::instrument(skip(current_user, db))]
pub async fn account_deactivate(
    current_user: CurrentUser,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    User::delete(current_user.id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |_| Ok(StatusCode::NO_CONTENT),
    )
}
