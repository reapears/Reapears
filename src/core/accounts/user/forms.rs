//! User forms impls

use axum::async_trait;
use serde::Deserialize;
use time::{Date, OffsetDateTime};
use uuid::Uuid;
use validator::Validate;

use crate::{
    accounts::emails::{forms::EmailInsertData, EmailModel},
    auth::{hash_password, TokenHash},
    db,
    endpoint::{EndpointRejection, EndpointResult, ModelId, ValidateForm},
    error::ServerResult,
    server::state::ServerState,
};

/// User sign-up form
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignUpForm {
    #[validate(length(min = 1, max = 16))]
    pub first_name: String,

    #[validate(length(max = 16))]
    pub last_name: Option<String>,

    #[validate(email)]
    pub email: String,

    #[validate(length(
        min = 7,
        max = 14,
        message = "Password length must be at least 7 characters long and 14 characters max"
    ))]
    pub password: String,
}

/// Signup cleaned data
#[derive(Debug, Clone)]
pub struct SignUpData {
    pub id: Uuid,
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
    /// Convert `Self` into `SignUpData`
    ///
    /// # Errors
    ///
    /// Return an error if failed to hash a password
    pub async fn try_data(self, email_token: TokenHash) -> ServerResult<SignUpData> {
        let data = SignUpData {
            id: db::model_id(),
            first_name: self.first_name,
            last_name: self.last_name,
            email: EmailInsertData::new(self.email.to_lowercase(), email_token),
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
impl ValidateForm<ServerState> for SignUpForm {
    #[tracing::instrument(skip(self, state), name = "Validate SignUpForm")]
    async fn validate_form(
        self,
        state: &ServerState,
        _model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        match self.validate() {
            Ok(()) => {
                let db = state.database.clone();
                let email = self.email.clone();
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

                Ok(self)
            }
            Err(err) => Err(EndpointRejection::BadRequest(err.to_string().into())),
        }
    }
}

/// User account lock form
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AccountLockForm {
    pub user_id: Uuid,
    pub account_locked_reason: String,
    pub account_locked_until: Option<Date>,
}

/// User account lock cleaned data
#[derive(Debug, Clone, Deserialize)]
pub struct AccountLockData {
    pub user_id: Uuid,
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

#[async_trait]
impl ValidateForm<ServerState> for AccountLockForm {
    #[tracing::instrument(skip(self, state), name = "Validate AccountLockForm")]
    async fn validate_form(
        self,
        state: &ServerState,
        _model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        match self.validate() {
            Ok(()) => {
                let db = state.database.clone();
                helpers::validate_user_exists(self.user_id, db).await?;
                Ok(self)
            }
            Err(err) => Err(EndpointRejection::BadRequest(err.to_string().into())),
        }
    }
}

/// User account unlock form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountUnlockForm {
    pub user_id: Uuid,
}

#[async_trait]
impl ValidateForm<ServerState> for AccountUnlockForm {
    #[tracing::instrument(skip(self, state), name = "Validate AccountLockForm")]
    async fn validate_form(
        self,
        state: &ServerState,
        _model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        let db = state.database.clone();
        helpers::validate_user_exists(self.user_id, db).await?;
        Ok(self)
    }
}

mod helpers {

    use crate::{
        endpoint::{EndpointRejection, EndpointResult},
        server::state::DatabaseConnection,
    };
    use uuid::Uuid;

    /// Validate `user_id` exists
    pub async fn validate_user_exists(id: Uuid, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM accounts.users user_
                    WHERE user_.id = $1
                ) AS "exists!"
            "#,
            id,
        )
        .fetch_one(&db.pool)
        .await
        {
            // Returns ok if user id exists
            Ok(row) => {
                if row.exists {
                    Ok(())
                } else {
                    tracing::error!("User id: '{}' does not exists.", id);
                    Err(EndpointRejection::BadRequest(
                        "Sorry! Account not found.".into(),
                    ))
                }
            }
            Err(err) => {
                tracing::error!("Database error: {}", err);
                Err(EndpointRejection::internal_server_error())
            }
        }
    }
}
