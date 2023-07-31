//! User database impls

use time::OffsetDateTime;

use crate::{
    accounts::{
        emails::EmailModel,
        user_profile::{delete_user_photo, models::UserProfile},
    },
    endpoint::EndpointRejection,
    error::{ServerError, ServerResult},
    server::state::DatabaseConnection,
    services::produce::harvest::delete_harvest_photos,
    types::ModelID,
    types::Pagination,
};

use super::{
    forms::{AccountLockData, SignUpData},
    models::{User, UserIndex, UserList},
    utils::{
        archive_user_farms, archive_user_harvests, archive_user_locations, delete_user_farms,
        delete_user_harvests, delete_user_locations, get_user_photo, session_delete, user_delete,
        user_harvest_photos, user_is_farmer,
    },
};

impl User {
    /// Fetches user records from the database
    #[tracing::instrument(skip(db))]
    pub async fn records(pagination: Pagination, db: DatabaseConnection) -> ServerResult<UserList> {
        let (offset, limit) = pagination.offset_limit();
        match sqlx::query!(
            r#"
                SELECT user_.id AS user_id,
                    user_.first_name AS user_first_name,
                    user_.last_name AS user_last_name,
                    profile.photo AS user_photo
                FROM accounts.users user_
                LEFT JOIN accounts.user_profiles profile
                    ON user_.id = profile.user_id
                ORDER BY user_.last_name, user_.last_name
                LIMIT $1
                OFFSET $2;
            "#,
            limit,
            offset
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let users = records
                    .into_iter()
                    .map(|rec| {
                        UserIndex::from_row(
                            rec.user_id.into(),
                            rec.user_first_name,
                            rec.user_last_name,
                            rec.user_photo,
                        )
                    })
                    .collect();

                Ok(users)
            }
            Err(err) => {
                tracing::error!("Database error, failed to fetch users: {}", err);
                Err(err.into())
            }
        }
    }

    /// Inserts  new user into the database
    #[tracing::instrument(skip(db, user))]
    pub async fn insert(user: SignUpData, db: DatabaseConnection) -> ServerResult<ModelID> {
        let mut tx = db.pool.begin().await?;
        let user_id = user.id;
        match sqlx::query!(
            r#"
                INSERT INTO accounts.users(
                    id, 
                    first_name, 
                    last_name, 
                    phc_string, 
                    is_staff,
                    is_superuser,
                    is_farmer, 
                    date_joined, 
                    account_locked
                )
                 VALUES($1, $2, $3, $4, $5, $6, false, $7, $8);
            "#,
            user_id.0,
            user.first_name,
            user.last_name,
            user.phc_string,
            user.is_staff,
            user.is_superuser,
            user.date_joined,
            user.account_locked,
        )
        .execute(&mut *tx)
        .await
        {
            Ok(result) => {
                tracing::debug!(
                    "User inserted successfully, but transaction not committed: {:?}",
                    result
                );
                // Insert user email
                EmailModel::insert(user_id, user.email, &mut tx).await?;

                // Insert user profile
                UserProfile::insert_default(user_id, &mut tx).await?;

                tx.commit().await?; // Commit transaction

                tracing::debug!("User inserted successfully");
                Ok(user_id)
            }
            Err(err) => {
                // Handle database constraint error
                handle_user_database_error(&err)?;

                tracing::error!("Database error, failed to insert new user: {}", err);
                Err(err.into())
            }
        }
    }

    /// Fetches `user_id` and `first_name` by email from the database
    pub async fn find_by_email(
        email: String,
        db: DatabaseConnection,
    ) -> ServerResult<Option<(ModelID, String)>> {
        match sqlx::query!(
            r#"
                SELECT user_.id AS user_id,
                    user_.first_name
                FROM accounts.emails address
                LEFT JOIN accounts.users user_
                    ON address.user_id = user_.id
                WHERE LOWER(address.email) = LOWER($1);
            "#,
            email
        )
        .fetch_optional(&db.pool)
        .await
        {
            Ok(rec) => Ok(rec.map(|rec| (rec.user_id.into(), rec.first_name))),
            Err(err) => {
                tracing::error!("Database error, failed to fetch user by email: {}", err);
                Err(err.into())
            }
        }
    }

    /// Add or remove user is superuser
    pub async fn set_superuser(
        user_id: ModelID,
        is_superuser: bool,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE accounts.users user_
                SET is_superuser = $1

                WHERE user_.id = $2
            "#,
            is_superuser,
            user_id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(_result) => Ok(()),
            Err(err) => {
                // Handle database constraint error
                handle_user_database_error(&err)?;

                tracing::error!(
                    "Database error, failed to set user is superuser={is_superuser}: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Add or remove user is staff
    pub async fn set_staff(
        user_id: ModelID,
        is_staff: bool,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE accounts.users user_
                SET is_staff = $1

                WHERE user_.id = $2
            "#,
            is_staff,
            user_id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(_result) => Ok(()),
            Err(err) => {
                // Handle database constraint error
                handle_user_database_error(&err)?;

                tracing::error!(
                    "Database error, failed to set user is superuser={is_staff}: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Locks user account
    #[tracing::instrument(skip(db))]
    pub async fn lock_account(values: AccountLockData, db: DatabaseConnection) -> ServerResult<()> {
        let mut tx = db.pool.begin().await?;
        match sqlx::query!(
            r#"
                UPDATE accounts.users user_
                SET account_locked = TRUE,
                    account_locked_reason = $1,
                    account_locked_until = $2
                WHERE user_.id = $3;
               "#,
            values.account_locked_reason,
            values.account_locked_until,
            values.user_id.0
        )
        .execute(&mut *tx)
        .await
        {
            Ok(result) => {
                // Delete user sessions so they cannot continue
                // using their account after it's locked
                session_delete(values.user_id, &mut tx).await?;
                tx.commit().await?;

                tracing::debug!("Account locked successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_user_database_error(&err)?;

                tracing::error!("Database error, Failed to lock an account: {}", err);
                Err(err.into())
            }
        }
    }

    /// Unlock user account
    #[tracing::instrument(skip(db))]
    pub async fn unlock_account(user_id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE accounts.users user_
                SET account_locked = FALSE,
                    account_locked_reason = NULL,
                    account_locked_until = NULL
                WHERE user_.id = $1;
                "#,
            user_id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Account unlocked successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_user_database_error(&err)?;

                tracing::error!("Database error, Failed to unlock an account: {}", err);
                Err(err.into())
            }
        }
    }

    /// Deletes user from the database
    #[tracing::instrument(skip(db))]
    pub async fn delete(id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        let pool = db.clone();
        let profile_photo = get_user_photo(id, pool.clone()).await?;
        let mut tx = db.pool.begin().await?;

        // Clean up user's farms-location-harvests
        if user_is_farmer(id, pool.clone()).await? {
            let image_paths = user_harvest_photos(id, pool).await?;

            let deleted_at = OffsetDateTime::now_utc();

            // Cleanup user farms harvests
            archive_user_harvests(id, deleted_at, &mut tx).await?;
            delete_user_harvests(id, deleted_at, &mut tx).await?;

            // Cleanup farm locations
            archive_user_locations(id, deleted_at, &mut tx).await?;
            delete_user_locations(id, &mut tx).await?;

            //Cleanup farms
            archive_user_farms(id, deleted_at, &mut tx).await?;
            delete_user_farms(id, &mut tx).await?;

            // Cleanup active harvest images
            tokio::spawn(
                async move { delete_harvest_photos(image_paths.into_iter().flatten()).await },
            );
        }

        user_delete(id, &mut tx).await?;
        tx.commit().await?;
        tracing::debug!("User::delete, transaction committed successfully.");

        // Delete profile-phot
        if let Some(photo) = profile_photo {
            tokio::spawn(async move { delete_user_photo(&photo).await });
        }

        Ok(())
    }

    /// Deletes unconfirmed user account from the database
    pub async fn delete_unverified(id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                DELETE FROM accounts.users user_
                WHERE user_.id = $1
            "#,
            id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::error!("Unverified user deleted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_user_database_error(&err)?;

                tracing::error!("Database error, failed to delete unverified user: {}", err);
                Err(err.into())
            }
        }
    }
}

/// Handle user database constraints errors
#[allow(clippy::cognitive_complexity)]
pub fn handle_user_database_error(err: &sqlx::Error) -> ServerResult<()> {
    if matches!(err, &sqlx::Error::RowNotFound) {
        tracing::error!("Database error, user not found. {:?}", err);
        return Err(ServerError::rejection(EndpointRejection::NotFound(
            "User not found.".into(),
        )));
    }

    Ok(())
}
