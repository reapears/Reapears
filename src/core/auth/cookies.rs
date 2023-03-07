//! Cookie names definitions

/// Cookie used for user authentication
pub const SESSION_TOKEN: &str = "auth_token";

/// Cookie used when a user want to perform sensitive tasks
/// that require them to authenticate with their password first
pub const PASSWORD_VERIFIED: &str = "pwd_auth";
