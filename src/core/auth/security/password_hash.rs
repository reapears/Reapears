//! Password hashing and verification impls

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::error::{ServerError, ServerResult};

/// Hashes a password and return a PHC string `using argon2 with default params`
#[tracing::instrument(skip(password))]
pub async fn hash_password(password: String) -> ServerResult<String> {
    match tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hasher_result: ServerResult<String> =
            match argon2.hash_password(password.as_bytes(), &salt) {
                Ok(hash) => Ok(hash.to_string()),
                Err(err) => {
                    tracing::error!("Error occurred while hashing a password: {}", err);
                    Err(err.into())
                }
            };
        hasher_result
    })
    .await
    {
        Ok(hasher_result) => hasher_result,
        Err(err) => Err(ServerError::internal(Box::new(err))),
    }
}

/// Verifies a `password` against a `PHC string` using `argon2 with default params`
///
/// Returns true if the password is correct
#[tracing::instrument(skip(password, phc_string))]
pub async fn verify_password(password: &str, phc_string: String) -> ServerResult<bool> {
    let password = password.to_owned();
    match tokio::task::spawn_blocking(move || {
        let hash_result: ServerResult<bool> = match PasswordHash::new(&phc_string) {
            Ok(password_hash) => {
                let matches = Argon2::default()
                    .verify_password(password.as_bytes(), &password_hash)
                    .is_ok();
                Ok(matches)
            }
            Err(err) => {
                tracing::error!("Error occurred while verifying a password: {}", err);
                Err(err.into())
            }
        };
        hash_result
    })
    .await
    {
        Ok(hasher_result) => hasher_result,
        Err(err) => Err(ServerError::internal(Box::new(err))),
    }
}
