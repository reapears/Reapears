//! Password hashing and verification impls

use tokio::task::spawn_blocking;

use password_auth;

use crate::error::{ServerError, ServerResult};

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
pub async fn verify_password(password: &str, phc_string: String) -> ServerResult<bool> {
    let password = password.to_owned();
    match spawn_blocking(move || {
        match password_auth::verify_password(password.as_bytes(), &phc_string) {
            Ok(()) => Ok(true),
            Err(password_auth::VerifyError::PasswordInvalid) => Ok(false),
            Err(err) => {
                tracing::error!("Password verification error: {}", err);
                Err(err.into())
            }
        }
    })
    .await
    {
        Ok(result) => result,
        Err(err) => Err(ServerError::internal(Box::new(err))),
    }
}
