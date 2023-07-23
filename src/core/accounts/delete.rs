//! Account delete impls

use time::{Duration, OffsetDateTime};
use tokio::task::JoinSet;

use crate::{
    accounts::user::models::User, error::ServerResult, server::state::DatabaseConnection,
    types::ModelID,
};

/// A list of user ids that requested for account deletion.
pub type AccountDeleteRequests = Vec<ModelID>;

/// Max numbers of days a user has before their account deleted permanently.
pub const MAX_DAYS_TO_DELETE_ACCOUNT: u8 = 90;

/// A Handler for accounts that requested to be deleted.
#[derive(Debug, Clone)]
pub struct AccountDelete;

impl AccountDelete {
    /// Permanently delete all the accounts the requested for deletion
    pub async fn permanently_delete_accounts(db: DatabaseConnection) {
        // Get account delete requests
        let accounts = match Self::records(db.clone()).await {
            Ok(records) => records,
            Err(_err) => {
                tracing::error!(
                    "Account could not be permanently deleted; failed to fetch delete requests."
                );
                return;
            }
        };
        // Spawn account delete task for each user_id
        let mut task = JoinSet::new();
        for user_id in accounts {
            let db = db.clone();
            task.spawn(async move { User::delete(user_id, db).await });
        }
        // Wait for tasks to run to completion; We don't care about the results.
        while let Some(_res) = task.join_next().await {}
    }

    /// Fetches account delete request records from the database
    #[tracing::instrument(name = "Fetch account delete requests", skip(db))]
    pub async fn records(db: DatabaseConnection) -> ServerResult<AccountDeleteRequests> {
        match sqlx::query!(
            r#"
                SELECT  delete_request.user_id,
                     delete_request.delete_request_at
                FROM accounts.account_delete_requests delete_request
            "#
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let now = OffsetDateTime::now_utc().date();

                let delete_requests = records
                    .into_iter()
                    // Filter only account that can be deleted;
                    // such that the delete_request_at <= now_utc()
                    .filter(|rec| {
                        let request_date = rec.delete_request_at
                            - Duration::days(i64::from(MAX_DAYS_TO_DELETE_ACCOUNT));
                        request_date.date() <= now
                    })
                    .map(|rec| ModelID::from(rec.user_id))
                    .collect();

                Ok(delete_requests)
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to fetch account delete requests: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Insert user account delete request into the database
    #[tracing::instrument(skip(db), name = "Insert account delete request")]
    pub async fn insert(user_id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                INSERT INTO accounts.account_delete_requests(
                    user_id, 
                    delete_request_at
                )
                 VALUES($1, $2);
              "#,
            user_id.0,
            OffsetDateTime::now_utc(),
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Account delete request inserted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to insert account delete request: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Deletes account delete request from the database.
    ///
    /// This is usually done when a user logged-in into their account
    /// before the time to delete permanently has passed
    #[tracing::instrument(name = "Delete account delete request", skip(db))]
    pub async fn delete_request(id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                 DELETE FROM accounts.account_delete_requests
                 WHERE user_id = $1
            "#,
            id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Account delete request deleted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to delete account delete request: {}",
                    err
                );
                Err(err.into())
            }
        }
    }
}
