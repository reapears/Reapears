//! Location helpers impls

use time::OffsetDateTime;

use crate::{
    error::ServerResult, server::state::DatabaseConnection,
    services::produce::harvest::harvest_max_age, types::ModelID,
};

use super::db::handle_location_database_error;

/// Delete location from the database
///
/// # Errors
///
/// Return database error
pub async fn delete_location(
    location_id: ModelID,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<()> {
    match sqlx::query!(
        r#"
            DELETE FROM services.locations location_
            WHERE location_.id = $1
        "#,
        location_id.0
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "Location deleted, but transaction not committed: {:?}",
                result
            );
            Ok(())
        }
        Err(err) => {
            // Handle database constraint error
            handle_location_database_error(&err)?;

            tracing::debug!("Database error, failed to delete location: {}", err);
            Err(err.into())
        }
    }
}

/// Archive location from the database
///
/// # Errors
///
/// Return database error
pub async fn archive_location(
    location_id: ModelID,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<()> {
    match sqlx::query!(
        r#"
            UPDATE services.locations location_
            SET deleted = true,
                deleted_at = $1
            WHERE location_.id = $2
        "#,
        OffsetDateTime::now_utc().date(),
        location_id.0
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "Location archived, but transaction not committed: {:?}",
                result
            );
            Ok(())
        }
        Err(err) => {
            // Handle database constraint error
            handle_location_database_error(&err)?;

            tracing::error!("Database error, failed to archive location: {}", err);
            Err(err.into())
        }
    }
}

/// Fetch location active harvests images
///
/// # Errors
///
/// Return database error
pub async fn location_harvest_photos(
    location_id: ModelID,
    db: DatabaseConnection,
) -> ServerResult<Vec<Vec<String>>> {
    match sqlx::query!(
        r#"
            SELECT harvest.images
            FROM services.active_harvests harvest
            WHERE harvest.location_id = $1
        "#,
        location_id.0
    )
    .fetch_all(&db.pool)
    .await
    {
        Ok(records) => Ok(records.into_iter().filter_map(|rec| rec.images).collect()),
        Err(err) => {
            tracing::error!(
                "Database error, failed to fetch location harvest images: {}",
                err
            );
            Err(err.into())
        }
    }
}

/// Find location archived harvest count
///
/// # Errors
///
/// Return database error
pub async fn location_archived_harvest_count(
    location_id: ModelID,
    db: DatabaseConnection,
) -> ServerResult<i64> {
    match sqlx::query!(
        r#"
            SELECT COUNT(harvest.id) AS "harvest_count!"
            FROM services.locations location_
            LEFT JOIN services.harvests harvest
                ON location_.id = harvest.location_id
            
            WHERE location_.id = $1 AND harvest.finished = true;
        "#,
        location_id.0
    )
    .fetch_one(&db.pool)
    .await
    {
        Ok(rec) => Ok(rec.harvest_count),
        Err(err) => {
            tracing::error!(
                "Database error, failed to fetch location active harvests count: {}",
                err
            );
            Err(err.into())
        }
    }
}

/// Delete location active harvests
///
/// # Errors
///
/// Return database error
pub async fn delete_location_harvests(
    location_id: ModelID,
    finished_at: OffsetDateTime,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> ServerResult<u64> {
    let max_age = harvest_max_age(finished_at)?;
    match sqlx::query!(
        r#"
            DELETE FROM services.harvests harvest
            
            WHERE harvest.location_id = $1
                AND (
                    harvest.harvest_date > $2 OR 
                    harvest.created_at > $3
                )
        "#,
        location_id.0,
        finished_at.date(),
        max_age,
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "Location active harvests deleted, but transaction not committed: {:?}",
                result
            );
            Ok(result.rows_affected())
        }
        Err(err) => {
            tracing::error!("Database error, failed to delete location harvests");
            Err(err.into())
        }
    }
}

/// Archive location active harvests
///
/// # Errors
///
/// Return database error
pub async fn archive_location_harvests(
    location_id: ModelID,
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

            WHERE harvest.location_id = $2
                AND NOT(
                    harvest.harvest_date > $1 OR 
                    harvest.created_at > $3
                )
        "#,
        finished_at.date(),
        location_id.0,
        max_age,
    )
    .execute(&mut **tx)
    .await
    {
        Ok(result) => {
            tracing::trace!(
                "Location active harvests archived, but transaction not committed: {:?}",
                result
            );
            Ok(result.rows_affected())
        }
        Err(err) => {
            tracing::error!("Database error, failed to archive location harvests");
            Err(err.into())
        }
    }
}
