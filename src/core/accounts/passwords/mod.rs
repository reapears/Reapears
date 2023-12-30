//! Password related utilities impls

use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};

use crate::{
    auth::{cookies::PASSWORD_VERIFIED, verify_password},
    error::{ServerError, ServerResult},
    server::state::DatabaseConnection,
    types::ModelID,
};

pub mod db;
pub mod forms;
pub mod handlers;

/// Password database model
#[derive(Debug)]
pub struct PasswordModel;

/// Check if the user password is correct
///
/// return false if`user_id`d is not found in the database
///
/// # Error
///
/// Return database error
pub async fn check_password(
    user_id: ModelID,
    password: String,
    db: DatabaseConnection,
) -> ServerResult<()> {
    let Some(phc_string) = PasswordModel::find(user_id, db).await? else {
        tracing::error!("Database error, user could not be found");
        return Err(ServerError::new("User could not be found"));
    };
    verify_password(&password, phc_string).await
}

/// Checks if password reset token expired
///
/// Return true if the token has expired
#[must_use]
pub fn password_reset_token_expired(created_at: time::OffsetDateTime) -> bool {
    let now = time::OffsetDateTime::now_utc();
    let threshold = now - time::Duration::minutes(crate::PASSWORD_TOKEN_EXPIRY_TIME);
    created_at < threshold
}

/// Gets is password verified from the cookie jar
#[must_use]
pub fn get_password_verified(jar: &PrivateCookieJar) -> Option<bool> {
    let password_verified: Option<bool> = jar
        .get(PASSWORD_VERIFIED)
        .map(|cookie| cookie.value().to_owned())
        .map(|s| str_to_bool(&s));
    password_verified
}

/// Adds password verified cookie
fn add_password_verified_cookie(jar: PrivateCookieJar) -> PrivateCookieJar {
    let mut password_cookie = Cookie::new(PASSWORD_VERIFIED, "true");
    password_cookie.set_path("/");
    jar.add(password_cookie)
}

/// Removes password verified cookie
#[must_use]
pub fn remove_password_verified_cookie(jar: PrivateCookieJar) -> PrivateCookieJar {
    let jar = jar.remove(Cookie::build(PASSWORD_VERIFIED));
    jar
}

/// Convert str to bool
fn str_to_bool(s: &str) -> bool {
    let s = s.to_lowercase();
    matches!(s.as_str(), "true")
}

/*
Hello user,
 A request has been received to change the password for your reapears account
 <link>

 Thanks,
 The Reapears Team

 If you did not request a password reset, you can safely ignore this email.
 Only the person with access to your email can reset your account password.
*/
