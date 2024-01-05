//! User forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Json, Request},
};
use serde::Deserialize;
use time::{Date, OffsetDateTime};

use crate::{
    accounts::emails::{forms::EmailInsertData, EmailModel},
    auth::{hash_password, TokenHash},
    endpoint::{
        validators::{TransformString, ValidateString},
        EndpointRejection, EndpointResult,
    },
    error::ServerResult,
    server::state::ServerState,
    types::ModelID,
};

/// User sign-up form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignUpForm {
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: String,
    pub password: String,
}

/// Signup cleaned data
#[derive(Debug, Clone)]
pub struct SignUpData {
    pub id: ModelID,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: EmailInsertData,
    pub phc_string: String,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub date_joined: OffsetDateTime,
    pub account_locked: bool,
}

impl SignUpForm {
    /// Validates sign-up form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.first_name
            .validate_len(3, 24, "first name must be at most 24 characters")?;

        if let Some(ref last_name) = self.last_name {
            last_name.validate_len(0, 24, "last name must be at most 24 characters")?;
        }

        self.email.validate_email()?;

        self.password.validate_len(
            6,
            24,
            "password must be as least 6 character and at most 24 characters long",
        )?;

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.first_name = self.first_name.clean();
        self.last_name = self.last_name.as_ref().map(|last_name| last_name.clean());
        self.email = self.email.clean().to_ascii_lowercase();
    }

    /// Convert `Self` into `SignUpData`
    ///
    /// # Errors
    ///
    /// Return an error if failed to hash a password
    pub async fn try_data(self, email_token: TokenHash) -> ServerResult<SignUpData> {
        let data = SignUpData {
            id: ModelID::new(),
            first_name: self.first_name,
            last_name: self.last_name,
            email: EmailInsertData::new(self.email, email_token),
            phc_string: hash_password(self.password).await?,
            is_staff: false,
            is_superuser: false,
            date_joined: OffsetDateTime::now_utc(),
            account_locked: false,
        };
        Ok(data)
    }
}

#[async_trait]
impl FromRequest<ServerState> for SignUpForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut signup) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        signup.validate()?;

        let db = state.database();
        let email = signup.email.clone();
        if EmailModel::exists_and_verified(email.clone(), db.clone()).await? {
            // A redirect to login perhaps
            return Err(EndpointRejection::BadRequest(
                "Account exists already!".into(),
            ));
        }

        // If the user try to sign up again with the unverified email
        // Delete the existing record and continue
        if EmailModel::exists_and_unverified(email.clone(), db.clone()).await? {
            EmailModel::delete_unverified(email, db).await?;
        }

        Ok(signup)
    }
}

// ===== Account Lock impls =====

/// User account lock form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountLockForm {
    pub user_id: ModelID,
    pub account_locked_reason: String,
    pub account_locked_until: Option<Date>,
}

/// User account lock cleaned data
#[derive(Debug, Clone, Deserialize)]
pub struct AccountLockData {
    pub user_id: ModelID,
    pub account_locked_reason: String,
    pub account_locked_until: Option<Date>,
}

impl From<AccountLockForm> for AccountLockData {
    fn from(form: AccountLockForm) -> Self {
        Self {
            user_id: form.user_id,
            account_locked_reason: form.account_locked_reason,
            account_locked_until: form.account_locked_until,
        }
    }
}

impl AccountLockForm {
    /// Validates account lock form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.account_locked_reason.validate_len(
            3,
            512,
            "Account locked reason must be at most 512 characters",
        )?;

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.account_locked_reason = self.account_locked_reason.clean();
    }
}

#[async_trait]
impl FromRequest<ServerState> for AccountLockForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut account_lock) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        account_lock.validate()?;

        Ok(account_lock)
    }
}

// ===== User id form impls =====

/// User id form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIdForm {
    pub user_id: ModelID,
}

#[async_trait]
impl FromRequest<ServerState> for UserIdForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(account_unlock) = Json::<Self>::from_request(req, state).await?;

        Ok(account_unlock)
    }
}
