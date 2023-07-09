//! Harvest helpers impls

use camino::Utf8PathBuf;
use time::{Date, Duration, OffsetDateTime};
use tokio::task::JoinSet;

use crate::{
    error::{ServerError, ServerResult},
    files,
    server::state::DatabaseConnection,
    settings::HARVEST_UPLOAD_DIR,
    types::ModelID,
};

use super::HARVEST_MAX_AGE_TO_ARCHIVE;

/// find harvest from the database for deletion
///
/// # Errors
///
/// Return database error
pub async fn find_delete_harvest(
    harvest_id: ModelID,
    db: DatabaseConnection,
) -> ServerResult<DeleteHarvest> {
    match sqlx::query!(
        r#"
            SELECT harvest.id,
                harvest.available_at,
                harvest.created_at,
                harvest.images
            FROM services.harvests harvest
            WHERE harvest.id = $1;
        "#,
        harvest_id.0
    )
    .fetch_one(&db.pool)
    .await
    {
        Ok(rec) => Ok(DeleteHarvest {
            id: rec.id.into(),
            available_at: rec.available_at,
            created_at: rec.created_at,
            images: rec.images,
        }),
        Err(err) => {
            tracing::error!(
                "Database error, failed to fetch harvest for deletion: {}",
                err
            );
            Err(err.into())
        }
    }
}

/// A minimal harvest used for deletion
#[derive(Debug, Clone)]
pub struct DeleteHarvest {
    pub id: ModelID,
    pub available_at: Date,
    pub created_at: OffsetDateTime,
    pub images: Option<Vec<String>>,
}

/// Delete harvest from the database
///
/// # Errors
///
/// Return database error
async fn delete_harvest(harvest_id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
    match sqlx::query!(
        r#"
            DELETE FROM services.harvests harvest
            WHERE harvest.id = $1
        "#,
        harvest_id.0
    )
    .execute(&db.pool)
    .await
    {
        Ok(result) => {
            tracing::debug!("Harvest deleted successfully: {:?}", result);
            Ok(())
        }
        Err(err) => {
            tracing::error!("Database error, failed to delete harvest: {}", err);
            Err(err.into())
        }
    }
}

/// Archive harvest in the database
///
/// # Errors
///
/// Return database error
async fn archive_harvest(harvest_id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
    match sqlx::query!(
        r#"
            UPDATE services.harvests harvest
            SET finished = true,
                images = NULL,
                finished_at = $1
            WHERE harvest.id = $2
        "#,
        OffsetDateTime::now_utc().date(),
        harvest_id.0
    )
    .execute(&db.pool)
    .await
    {
        Ok(result) => {
            tracing::debug!("Harvest archived successfully: {:?}", result);
            Ok(())
        }
        Err(err) => {
            tracing::error!("Database error, failed to archive harvest: {}", err);
            Err(err.into())
        }
    }
}

/// Deletes or archives the harvest
///
/// # Errors
///
/// Return database error
pub async fn delete_or_archive_harvest(
    harvest: DeleteHarvest,
    db: DatabaseConnection,
) -> ServerResult<()> {
    let finished_at = OffsetDateTime::now_utc();
    if can_delete_harvest(harvest.available_at, harvest.created_at, finished_at)? {
        delete_harvest(harvest.id, db).await
    } else {
        archive_harvest(harvest.id, db).await
    }
}

/// Return whether or not the harvest can be deleted,
/// return true if the harvest can be deleted.
///
/// # Errors
///
/// Return an error if failed to calculate harvest max age
fn can_delete_harvest(
    available_at: Date,
    created_at: OffsetDateTime,
    finished_at: OffsetDateTime,
) -> ServerResult<bool> {
    Ok(available_at > finished_at.date() || created_at > harvest_max_age(finished_at)?)
}

/// Calculate the harvest max age to archive
///
/// if the harvest `created_at` date is greater than max age to archive it will be deleted
///
/// # Errors
///
/// Return an error if failed to calculate harvest max age
pub fn harvest_max_age(finished_at: OffsetDateTime) -> ServerResult<OffsetDateTime> {
    finished_at
        .checked_sub(Duration::days(HARVEST_MAX_AGE_TO_ARCHIVE))
        .ok_or_else(|| {
            ServerError::new(
                "Harvest database delete error, failed calculate harvest max age to archive.",
            )
        })
}

/// Delete harvest images from file system
///
/// # Errors
///
/// Return an error if failed to delete files
pub async fn delete_harvest_photos(paths: Vec<String>) -> ServerResult<()> {
    let mut all_paths = Vec::with_capacity(10);
    for path in paths {
        let path = Utf8PathBuf::from(&path);
        let file_stem = path.file_stem().unwrap_or(path.as_str());
        for ext in files::IMAGE_FORMATS {
            all_paths.push(Utf8PathBuf::from(format!(
                "{HARVEST_UPLOAD_DIR}/{file_stem}.{ext}"
            )));
        }
    }
    files::delete_files(all_paths).await
}

/// Delete a vector of harvest images
///
/// Ignores the errors
pub async fn delete_harvest_photos_list(images: Vec<Vec<String>>) {
    // Delete harvests images
    let mut tasks = JoinSet::new();
    for paths in images {
        tasks.spawn(async move { delete_harvest_photos(paths).await });
    }
    // Wait for tasks to finish
    while let Some(res) = tasks.join_next().await {
        if res.is_err() {
            tracing::error!("Harvest delete images task error: {res:?}");
        }
    }
}
