//! User models impls

use serde::Serialize;
use uuid::Uuid;

/// A `Vec` of users
pub type UserList = Vec<UserIndex>;

/// The model representing a row in the `users` database table.
#[derive(Debug, Clone, Serialize)]
pub struct User;

/// A type returned by `user_list` handler.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIndex {
    pub id: Uuid,
    pub full_name: String,
    pub photo: Option<String>,
}

impl UserIndex {
    #[must_use]
    /// Creates a new `UserIndex` from the database row
    pub fn from_row(
        id: Uuid,
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

// /// Filters for Users
// #[derive(Debug, Clone)]
// pub enum UserFilter {
//     FirstNameEq(Vec<String>),
//     FirstNameNe(Vec<String>),
//     LastNameEq(Vec<String>),
//     LastNameNe(Vec<String>),
//     FullNameEq(Vec<String>),
//     FullNameNe(Vec<String>),

//     EmailEq(Vec<String>),
//     EmailNe(Vec<String>),
//     PhoneEq(Vec<String>),
//     PhoneNe(Vec<String>),

//     GenderEq(Vec<String>),
//     GenderNe(Vec<String>),

//     DateOfBirthLt(Date),
//     DateOfBirthEq(Vec<Date>),
//     DateOfBirthNe(Vec<Date>),
//     DateOfBirthGt(Date),

//     GroupIdEq(Vec<String>),
//     GroupIdNe(Vec<String>),
//     GroupNameEq(Vec<String>),
//     GroupNameNe(Vec<String>),

//     PermissionIdEq(Vec<String>),
//     PermissionIdNe(Vec<String>),
//     PermissionCodeNameEq(Vec<String>),
//     PermissionCodeNameNe(Vec<String>),

//     DateJoinedLt(Date),
//     DateJoinedEq(Vec<Date>),
//     DateJoinedNe(Vec<Date>),
//     DateJoinedGt(Date),

//     LastLoginLt(Date),
//     LastLoginEq(Vec<Date>),
//     LastLoginNe(Vec<Date>),
//     LastLoginGt(Date),

//     HasSessionEq,
//     HasSessionNe,
// }

// Concatenate two names together
#[allow(clippy::needless_pass_by_value)]
fn concat_names(first_name: String, last_name: Option<String>) -> String {
    let last_name = last_name.unwrap_or_default();
    format!("{last_name} {first_name}").trim().to_owned()
}
