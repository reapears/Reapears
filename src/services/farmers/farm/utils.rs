//! Farm helpers impls

use time::OffsetDateTime;

use crate::{
    error::ServerResult,
    files,
    server::state::DatabaseConnection,
    services::{
        farmers::location::{db::handle_location_database_error, forms::LocationInsertData},
        produce::harvest::harvest_max_age,
    },
    settings,
    types::ModelID,
};

use super::db::handle_farm_database_error;

/// Insert farm-location into the database
pub async fn location_insert(
    location: LocationInsertData,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<()> {
    match sqlx::query!(
        r#"
            INSERT INTO services.locations(
                id,
                farm_id,
                place_name,
                country_id,
                region_id,
                description,
                coords,
                deleted,
                created_at
            )
            VALUES($1, $2, $3, $4, $5, $6, $7, false, $8);
        "#,
        location.id.0,
        location.farm_id.0,
        location.place_name,
        location.country_id.0,
        location.region_id.0,
        location.description,
        location.coords,
        location.created_at,
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "Location successfully inserted, but transaction not committed: {:?}",
                result
            );
            Ok(())
        }
        Err(err) => {
            // Handle database constraint error
            handle_location_database_error(&err)?;

            tracing::error!("Database error, failed to insert location: {}", err);
            Err(err.into())
        }
    }
}

// ===== User =====

/// Update user `is_farmer` into the database
///
/// # Errors
///
/// Return database error
pub async fn update_user_is_farmer(
    is_farmer: bool,
    id: ModelID,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<()> {
    match sqlx::query!(
        r#"
            UPDATE accounts.users
            SET is_farmer = $1
            WHERE id = $2;
       "#,
        is_farmer,
        id.0,
    )
    .execute(&mut **tx)
    .await
    {
        Ok(_result) => Ok(()),
        Err(err) => {
            tracing::error!("Database error, failed to update user is_farmer: {}", err);
            Err(err.into())
        }
    }
}

/// Fetch farm count belonging to the user
///
/// # Errors
///
/// Return database error
pub async fn user_farm_count(
    farm_id: ModelID,
    db: DatabaseConnection,
) -> ServerResult<(Option<ModelID>, i64)> {
    match sqlx::query!(
        r#"
            SELECT farm.owner_id AS user_id,
                COUNT(farm.id) AS "farm_count!"
            FROM services.active_farms farm
            WHERE farm.owner_id
                IN (
                    SELECT farm.owner_id
                    FROM services.active_farms farm
                    WHERE farm.id = $1
                )
            GROUP BY farm.owner_id
        "#,
        farm_id.0,
    )
    .fetch_one(&db.pool)
    .await
    {
        Ok(rec) => Ok((rec.user_id.map(Into::into), rec.farm_count)),
        Err(err) => {
            tracing::error!("Database error, failed to fetch farm-count: {}", err);
            Err(err.into())
        }
    }
}

// ==== Farm =====

/// Delete farm from the database
///
/// # Errors
///
/// Return database error
pub async fn delete_farm(
    farm_id: ModelID,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<()> {
    match sqlx::query!(
        r#"
            DELETE FROM services.farms farm
            WHERE farm.id = $1
        "#,
        farm_id.0
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!("Farm deleted,  but transaction not committed: {:?}", result);
            Ok(())
        }
        Err(err) => {
            // Handle database constraint error
            handle_farm_database_error(&err)?;

            tracing::error!("Database error, failed to delete farm: {}", err);
            Err(err.into())
        }
    }
}

/// Archive farm in the database
///
/// # Errors
///
/// Return database error
pub async fn archive_farm(
    farm_id: ModelID,
    deleted_at: OffsetDateTime,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<()> {
    match sqlx::query!(
        r#"
            UPDATE services.farms farm
                SET deleted = true,
                deleted_at = $1
            WHERE farm.id = $2
        "#,
        deleted_at.date(),
        farm_id.0
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "Farm archived,  but transaction not committed: {:?}",
                result
            );
            Ok(())
        }
        Err(err) => {
            // Handle database constraint error
            handle_farm_database_error(&err)?;

            tracing::error!("Database error, failed to archive farm: {}", err);
            Err(err.into())
        }
    }
}

// ===== Location =====

/// Delete farm active locations
///
/// # Errors
///
/// Return database error
pub async fn delete_farm_locations(
    farm_id: ModelID,
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

                WHERE farm.id = $1
                GROUP BY location_.id
            )

            DELETE FROM services.locations location_

            WHERE location_.id IN(
                SELECT stat.location_id
                FROM location_stats stat
                WHERE stat.count = 0
            );
        "#,
        farm_id.0,
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "Farm active location deleted, but transaction not committed: {:?}",
                result
            );
            Ok(result.rows_affected())
        }
        Err(err) => {
            tracing::error!("Database error, failed to delete farm locations");
            Err(err.into())
        }
    }
}

/// Archive farm active locations
///
/// # Errors
///
/// Return database error
pub async fn archive_farm_locations(
    farm_id: ModelID,
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

                WHERE farm.id = $1
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
        farm_id.0,
        deleted_at.date(),
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "Farm active locations archived, but transaction not committed: {:?}",
                result
            );
            Ok(result.rows_affected())
        }
        Err(err) => {
            tracing::error!("Database error, failed to archive farm locations");
            Err(err.into())
        }
    }
}

// ===== Harvest =====

/// Delete farm active harvests
///
/// # Errors
///
/// Return database error
pub async fn delete_farm_harvests(
    farm_id: ModelID,
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
                WHERE location_.farm_id = $1
            )
            AND (
                harvest.harvest_date > $2 OR
                harvest.created_at > $3
            )
        "#,
        farm_id.0,
        finished_at.date(),
        max_age,
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "Farm active harvests deleted, but transaction not committed: {:?}",
                result
            );
            Ok(result.rows_affected())
        }
        Err(err) => {
            tracing::error!("Database error, failed to delete farm harvests");
            Err(err.into())
        }
    }
}

/// Archive farm active harvests
///
/// # Errors
///
/// Return database error
pub async fn archive_farm_harvests(
    farm_id: ModelID,
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
                WHERE location_.farm_id = $2
            )
            AND NOT(
                harvest.harvest_date > $1 OR
                harvest.created_at > $3
            )
        "#,
        finished_at.date(),
        farm_id.0,
        max_age,
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "Farm active harvests archived, but transaction not committed: {:?}",
                result
            );
            Ok(result.rows_affected())
        }
        Err(err) => {
            tracing::error!("Database error, failed to archive farm harvests");
            Err(err.into())
        }
    }
}

/// Find farm archived harvest count
///
/// # Errors
///
/// Return database error
pub async fn farm_archived_harvest_count(
    farm_id: ModelID,
    db: DatabaseConnection,
) -> ServerResult<i64> {
    match sqlx::query!(
        r#"
            SELECT COUNT(harvest.id) AS "harvest_count!"
            FROM services.harvests harvest

            WHERE harvest.location_id IN (
                SELECT location_.id
                FROM services.locations location_
                WHERE location_.farm_id = $1
            )
            AND harvest.finished = true;
        "#,
        farm_id.0
    )
    .fetch_one(&db.pool)
    .await
    {
        Ok(rec) => Ok(rec.harvest_count),
        Err(err) => {
            tracing::error!(
                "Database error, failed to fetch farm active harvests count: {}",
                err
            );
            Err(err.into())
        }
    }
}

/// Fetch farm active harvests images
///
/// # Errors
///
/// Return database error
pub async fn farm_harvest_images(
    farm_id: ModelID,
    db: DatabaseConnection,
) -> ServerResult<Vec<Vec<String>>> {
    match sqlx::query!(
        r#"
            SELECT harvest.images
            FROM services.active_harvests harvest

            WHERE harvest.location_id IN (
                SELECT location_.id
                FROM services.active_locations location_
                WHERE location_.farm_id = $1
            )
        "#,
        farm_id.0
    )
    .fetch_all(&db.pool)
    .await
    {
        Ok(records) => Ok(records.into_iter().filter_map(|rec| rec.images).collect()),
        Err(err) => {
            tracing::error!(
                "Database error, failed to fetch farm harvest images: {}",
                err
            );
            Err(err.into())
        }
    }
}

///  Delete farm logo fom the file system
///
/// # Errors
///
/// Return io error
pub async fn delete_farm_logo(file_name: &str) -> ServerResult<()> {
    let paths = files::saved_paths(settings::FARM_LOGO_UPLOAD_DIR, file_name);
    files::delete_files(paths).await
}
