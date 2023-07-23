//! Password hashing and verification impls

use tokio::task::spawn_blocking;

use password_auth;

use crate::error::{ServerError, ServerResult};

/// An error message for when a user entered a wrong password of username
pub const INVALID_CREDENTIALS: &str = "The username or password you provided is incorrect.";

/// Hashes a password and return a PHC string `using argon2 with default params`
#[tracing::instrument(skip(password))]
pub async fn hash_password(password: String) -> ServerResult<String> {
    spawn_blocking(move || password_auth::generate_hash(password.as_bytes()))
        .await
        .map_err(|err| ServerError::internal(Box::new(err)))
}

/// Verifies a `password` against a `PHC string` using `argon2 with default params`
///
/// Returns true if the password is correct
#[tracing::instrument(skip(password, phc_string))]
pub async fn verify_password(password: &str, phc_string: String) -> ServerResult<()> {
    let password = password.to_owned();
    match spawn_blocking(move || {
        password_auth::verify_password(password.as_bytes(), &phc_string).map_err(|err| match err {
            password_auth::VerifyError::PasswordInvalid => {
                ServerError::bad_request(INVALID_CREDENTIALS)
            }
            other_err @ password_auth::VerifyError::Parse(_) => {
                ServerError::internal(Box::new(other_err))
            }
        })
    })
    .await
    {
        Ok(result) => result,
        Err(err) => Err(ServerError::internal(Box::new(err))),
    }
}
