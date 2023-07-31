//! User models impls

use serde::Serialize;
use time::OffsetDateTime;

use crate::{
    accounts::emails::forms::EmailInsertData,
    auth::{hash_password, Token},
    server::state::DatabaseConnection,
    types::ModelID,
};

/// A `Vec` of users
pub type UserList = Vec<UserIndex>;

/// The model representing a row in the `users` database table.
#[derive(Debug, Clone, Serialize)]
pub struct User;

/// A type returned by `user_list` handler.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIndex {
    pub id: ModelID,
    pub full_name: String,
    pub photo: Option<String>,
}

impl UserIndex {
    #[must_use]
    /// Creates a new `UserIndex` from the database row
    pub fn from_row(
        id: ModelID,
        first_name: String,
        last_name: Option<String>,
        photo: Option<String>,
    ) -> Self {
        let full_name = concat_names(first_name, last_name);
        Self {
            id,
            full_name,
            photo,
        }
    }
}

// Concatenate two names together
#[allow(clippy::needless_pass_by_value)]
fn concat_names(first_name: String, last_name: Option<String>) -> String {
    let last_name = last_name.unwrap_or_default();
    format!("{last_name} {first_name}").trim().to_owned()
}

/// Creates superuser account
pub async fn create_unsecure_superuser(
    email: String,
    password: String,
    db: DatabaseConnection,
) -> ModelID {
    let email = EmailInsertData {
        email: email.trim().to_ascii_lowercase(),
        verified: true,
        token: Token::generate(32).hash,
        token_generated_at: OffsetDateTime::now_utc(),
    };
    let data = super::forms::SignUpData {
        id: ModelID::new(),
        first_name: String::new(),
        last_name: None,
        email,
        phc_string: hash_password(password.trim().to_owned()).await.unwrap(),
        is_staff: true,
        is_superuser: true,
        date_joined: OffsetDateTime::now_utc(),
        account_locked: false,
    };

    User::insert(data, db)
        .await
        .unwrap_or_else(|err| panic!("Failed to create superuser: {err}",))
}
