//! User helpers impls

use time::OffsetDateTime;

use crate::{
    error::ServerResult, server::state::DatabaseConnection,
    services::produce::harvest::harvest_max_age, types::ModelID,
};

/// Checks if user is farmer
///
/// # Errors
///
/// Return database error
pub async fn user_is_farmer(user_id: ModelID, db: DatabaseConnection) -> ServerResult<bool> {
    match sqlx::query!(
        r#"
            SELECT user_.is_farmer
            FROM accounts.users user_
            WHERE user_.id = $1
        "#,
        user_id.0
    )
    .fetch_one(&db.pool)
    .await
    {
        Ok(rec) => Ok(rec.is_farmer),
        Err(err) => {
            tracing::error!("Database error, failed check if user is_farmer: {}", err);
            Err(err.into())
        }
    }
}

/// Delete user from the database
///
/// # Errors
///
/// Return database error
pub async fn user_delete(
    user_id: ModelID,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<()> {
    match sqlx::query!(
        r#"
            DELETE FROM accounts.users user_
            WHERE user_.id = $1
        "#,
        user_id.0
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!("User deleted,  but transaction not committed: {:?}", result);
            Ok(())
        }
        Err(err) => {
            tracing::error!("Database error, failed to delete user: {}", err);
            Err(err.into())
        }
    }
}

/// Delete user sessions from the database
///
/// # Errors
///
/// Return database error
pub async fn session_delete(
    user_id: ModelID,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<()> {
    match sqlx::query!(
        r#"
            DELETE FROM auth.sessions session
            WHERE session.user_id = $1
        "#,
        user_id.0
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "User sessions deleted,  but transaction not committed: {:?}",
                result
            );
            Ok(())
        }
        Err(err) => {
            tracing::error!("Database error, failed to delete user session: {}", err);
            Err(err.into())
        }
    }
}

/// Fetches user profile photo path from the database
pub async fn get_user_photo(
    user_id: ModelID,
    db: DatabaseConnection,
) -> ServerResult<Option<String>> {
    match sqlx::query!(
        r#"
            SELECT profile.photo
            FROM accounts.user_profiles profile
            WHERE profile.user_id = $1
        "#,
        user_id.0
    )
    .fetch_one(&db.pool)
    .await
    {
        Ok(rec) => Ok(rec.photo),
        Err(err) => {
            if matches!(err, sqlx::Error::RowNotFound) {
                Ok(None)
            } else {
                tracing::error!("Database error, failed fetch user profile photo: {}", err);
                Err(err.into())
            }
        }
    }
}

// ---Farm---

/// Delete farm from the database
///
/// # Errors
///
/// Return database error
pub async fn delete_user_farms(
    user_id: ModelID,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<()> {
    match sqlx::query!(
        r#"
            WITH farm_stats AS(
                SELECT farm.id AS farm_id, COUNT(harvest.id)
                FROM services.active_farms farm
                LEFT JOIN services.locations location_
                    ON farm.id = location_.farm_id
                LEFT JOIN services.harvests harvest
                    ON location_.id = harvest.location_id

                WHERE farm.owner_id = $1
                GROUP BY farm.id
            )

            DELETE FROM services.farms farm

            WHERE farm.id IN(
                SELECT stat.farm_id
                FROM farm_stats stat
                WHERE stat.count = 0
            );
        "#,
        user_id.0
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "User farms deleted,  but transaction not committed: {:?}",
                result
            );
            Ok(())
        }
        Err(err) => {
            tracing::error!("Database error, failed to delete user farms: {}", err);
            Err(err.into())
        }
    }
}

/// Archive user farms in the database
///
/// # Errors
///
/// Return database error
pub async fn archive_user_farms(
    user_id: ModelID,
    deleted_at: OffsetDateTime,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<()> {
    match sqlx::query!(
        r#"
            WITH farm_stats AS(
                SELECT farm.id AS farm_id, COUNT(harvest.id)
                FROM services.active_farms farm
                LEFT JOIN services.locations location_
                    ON farm.id = location_.farm_id
                LEFT JOIN services.harvests harvest
                    ON location_.id = harvest.location_id

                WHERE farm.owner_id = $1
                GROUP BY farm.id
            )

            UPDATE services.farms farm
                SET deleted = true,
                    owner_id = NULL,
                    deleted_at = $2

            WHERE farm.id IN(
                SELECT stat.farm_id
                FROM farm_stats stat
                WHERE stat.count > 0
            );
        "#,
        user_id.0,
        deleted_at.date(),
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "User farm archived, but transaction not committed: {:?}",
                result
            );
            Ok(())
        }
        Err(err) => {
            tracing::error!("Database error, failed to archive user farms: {}", err);
            Err(err.into())
        }
    }
}

// ===== Location =====

/// Delete user active locations
///
/// # Errors
///
/// Return database error
pub async fn delete_user_locations(
    user_id: ModelID,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<u64> {
    match sqlx::query!(
        r#"
            WITH location_stats AS(
                SELECT location_.id AS location_id, COUNT(harvest.id)
                FROM services.active_farms farm
                LEFT JOIN services.active_locations location_
                    ON farm.id = location_.farm_id
                LEFT JOIN services.harvests harvest
                    ON location_.id = harvest.location_id

                WHERE farm.owner_id = $1
                GROUP BY location_.id
            )

            DELETE FROM services.locations location_
            
            WHERE location_.id IN(
                SELECT stat.location_id
                FROM location_stats stat
                WHERE stat.count = 0
            );
        "#,
        user_id.0,
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "User active location deleted, but transaction not committed: {:?}",
                result
            );
            Ok(result.rows_affected())
        }
        Err(err) => {
            tracing::error!("Database error, failed to delete user locations");
            Err(err.into())
        }
    }
}

/// Archive user active locations
///
/// # Errors
///
/// Return database error
pub async fn archive_user_locations(
    user_id: ModelID,
    deleted_at: OffsetDateTime,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<u64> {
    match sqlx::query!(
        r#"
            WITH location_stats AS(
                SELECT location_.id AS location_id, COUNT(harvest.id)
                FROM services.active_farms farm
                LEFT JOIN services.active_locations location_
                    ON farm.id = location_.farm_id
                LEFT JOIN services.harvests harvest
                    ON location_.id = harvest.location_id

                WHERE farm.owner_id = $1
                GROUP BY location_.id
            )

            UPDATE services.locations location_
                SET deleted = TRUE,
                    deleted_at = $2

            WHERE location_.id IN(
                SELECT stat.location_id
                FROM location_stats stat
                WHERE stat.count > 0
            );
        "#,
        user_id.0,
        deleted_at.date(),
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "User active locations archived, but transaction not committed: {:?}",
                result
            );
            Ok(result.rows_affected())
        }
        Err(err) => {
            tracing::error!("Database error, failed to archive user locations");
            Err(err.into())
        }
    }
}

// ===== Harvest =====

/// Delete user active harvests
///
/// # Errors
///
/// Return database error
pub async fn delete_user_harvests(
    user_id: ModelID,
    finished_at: OffsetDateTime,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<u64> {
    let max_age = harvest_max_age(finished_at)?;
    match sqlx::query!(
        r#"
            DELETE FROM services.harvests harvest
            
            WHERE harvest.location_id IN (
                SELECT location_.id
                FROM services.active_locations location_
                LEFT JOIN services.active_farms farm
                    ON location_.farm_id = farm.id
                WHERE farm.owner_id = $1
            )
            AND (
                harvest.harvest_date > $2 OR 
                harvest.created_at > $3
            )
        "#,
        user_id.0,
        finished_at.date(),
        max_age,
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "User active harvests deleted, but transaction not committed: {:?}",
                result
            );
            Ok(result.rows_affected())
        }
        Err(err) => {
            tracing::error!("Database error, failed to delete user harvests");
            Err(err.into())
        }
    }
}

/// Archive user active harvests
///
/// # Errors
///
/// Return database error
pub async fn archive_user_harvests(
    user_id: ModelID,
    finished_at: OffsetDateTime,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<u64> {
    let max_age = harvest_max_age(finished_at)?;
    match sqlx::query!(
        r#"
            UPDATE services.harvests harvest
            SET finished = true,
                images = NULL,
                finished_at = $1

            WHERE harvest.location_id IN (
                SELECT location_.id
                FROM services.active_locations location_
                LEFT JOIN services.active_farms farm
                    ON location_.farm_id = farm.id
                WHERE farm.owner_id = $2
            )
            AND NOT(
                harvest.harvest_date > $1 OR 
                harvest.created_at > $3
            )
        "#,
        finished_at.date(),
        user_id.0,
        max_age,
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "User active harvests archived, but transaction not committed: {:?}",
                result
            );
            Ok(result.rows_affected())
        }
        Err(err) => {
            tracing::error!("Database error, failed to archive user harvests");
            Err(err.into())
        }
    }
}

/// Fetch user active harvests images
///
/// # Errors
///
/// Return database error
pub async fn user_harvest_photos(
    user_id: ModelID,
    db: DatabaseConnection,
) -> ServerResult<Vec<Vec<String>>> {
    match sqlx::query!(
        r#"
            SELECT harvest.images
            FROM services.active_harvests harvest

            WHERE harvest.location_id IN (
                SELECT location_.id
                FROM services.active_locations location_
                LEFT JOIN services.active_farms farm
                    ON location_.farm_id = farm.id
                WHERE farm.owner_id = $1
            )
        "#,
        user_id.0
    )
    .fetch_all(&db.pool)
    .await
    {
        Ok(records) => Ok(records.into_iter().filter_map(|rec| rec.images).collect()),
        Err(err) => {
            tracing::error!(
                "Database error, failed to fetch user harvest images: {}",
                err
            );
            Err(err.into())
        }
    }
}
