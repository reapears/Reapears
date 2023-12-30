//! Cultivar database impl

use std::path::PathBuf;

use crate::{
    endpoint::EndpointRejection,
    error::{ServerError, ServerResult},
    files,
    server::state::DatabaseConnection,
    services::produce::harvest::models::HarvestIndex,
    types::{ModelID, ModelIdentifier, ModelIndex, Pagination},
};

use super::{
    forms::{CultivarInsertData, CultivarUpdateData},
    models::{Cultivar, CultivarIndex, CultivarList},
    utils::delete_cultivar_photo,
};

impl Cultivar {
    /// Fetches cultivar records from the database
    #[tracing::instrument(name = "Database::records-cultivar", skip(db))]
    pub async fn records(pg: Pagination, db: DatabaseConnection) -> ServerResult<CultivarList> {
        let (offset, limit) = pg.offset_limit();
        match sqlx::query!(
            r#"
                SELECT cultivar.id AS cultivar_id,
                    cultivar.name AS cultivar_name,
                    cultivar.image AS cultivar_image, 
                    cultivar_category.name AS cultivar_category,
                    (SELECT COUNT(harvest.id)
                     FROM services.harvests harvest
                     WHERE cultivar.id = harvest.cultivar_id) AS harvests_count
                FROM services.cultivars cultivar
                LEFT JOIN services.cultivar_categories cultivar_category
                    ON cultivar.category_id = cultivar_category.id

                ORDER BY cultivar.name
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
                let cultivars: Vec<_> = records
                    .into_iter()
                    .map(|rec| {
                        CultivarIndex::from_row(
                            rec.cultivar_id.into(),
                            rec.cultivar_name,
                            rec.cultivar_image,
                            rec.harvests_count,
                        )
                    })
                    .collect();

                Ok(cultivars)
            }
            Err(err) => {
                tracing::error!("Database error, failed to fetch cultivars: {}", err);
                Err(err.into())
            }
        }
    }

    /// Fetches cultivar detail from the database
    #[tracing::instrument(name = "Fetch Cultivar", skip(db))]
    pub async fn find(
        id: ModelID,
        pg: Option<Pagination>,
        db: DatabaseConnection,
    ) -> ServerResult<Option<Self>> {
        //NB! Don't forget to select harvests from services.active_harvests
        let (offset, limit) = pg.unwrap_or_default().offset_limit();
        match sqlx::query!(
            r#"
                SELECT cultivar.id AS cultivar_id,
                    cultivar.name AS cultivar_name,
                    cultivar.image AS cultivar_image, 
                    cultivar_category.name AS cultivar_category,
                    harvest.id AS "harvest_id?",
                    harvest.price AS "harvest_price?",
                    harvest.harvest_date AS "harvest_harvest_date?",
                    harvest.images AS harvest_images,
                    farm.name AS "farm_name?",
                    farm.logo AS farm_logo,
                    location_.place_name AS "location_place_name?",
                    location_.coords AS location_coords,
                    region.name AS "location_region?",
                    country.name AS "location_country?"
                FROM services.cultivars cultivar
                LEFT JOIN services.cultivar_categories cultivar_category
                    ON cultivar.category_id = cultivar_category.id
                LEFT JOIN services.active_harvests harvest
                    ON cultivar.id = harvest.cultivar_id
                LEFT JOIN services.locations location_
                    ON harvest.location_id = location_.id
                LEFT JOIN services.farms farm
                    ON location_.farm_id = farm.id
                LEFT JOIN services.regions region
                    ON location_.region_id = region.id
                LEFT JOIN services.countries country
                    ON location_.country_id = country.id

                WHERE cultivar.id = $1
                ORDER BY harvest.created_at
                LIMIT $2
                OFFSET $3;
            "#,
            id.0,
            limit,
            offset
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) if records.is_empty() => Ok(None),
            Ok(records) => {
                let first_rec = &records[0];

                let cultivar_id = first_rec.cultivar_id.into();
                let cultivar_name = first_rec.cultivar_name.clone();
                let cultivar_image = first_rec.cultivar_image.clone();
                let cultivar_category = first_rec.cultivar_category.clone();

                let harvests: Vec<_> = records
                    .into_iter()
                    .filter(|rec| rec.harvest_id.is_some())
                    .map(|rec| {
                        HarvestIndex::from_row(
                            rec.harvest_id.unwrap().into(),
                            rec.harvest_price.unwrap(),
                            rec.harvest_harvest_date.unwrap(),
                            rec.harvest_images,
                            rec.cultivar_name,
                            rec.cultivar_category,
                            rec.cultivar_image,
                            rec.location_place_name.unwrap(),
                            rec.location_region,
                            rec.location_country.unwrap(),
                            rec.location_coords,
                            rec.farm_name.unwrap(),
                            rec.farm_logo,
                            0.into(), // boost amount not important
                        )
                    })
                    .collect();
                let harvests = (!harvests.is_empty()).then_some(harvests);
                let cultivar = Self::from_row(
                    cultivar_id,
                    cultivar_name,
                    cultivar_category,
                    cultivar_image,
                    harvests,
                );
                Ok(Some(cultivar))
            }
            Err(err) => {
                tracing::error!("Database error, failed to fetch cultivar: {}", err);
                Err(err.into())
            }
        }
    }

    /// Inserts cultivar into the database
    #[tracing::instrument(name = "Insert Cultivar", skip(db, cultivar))]
    pub async fn insert(
        cultivar: CultivarInsertData,
        db: DatabaseConnection,
    ) -> ServerResult<ModelID> {
        match sqlx::query!(
            r#"
                INSERT INTO services.cultivars (
                    id, 
                    category_id, 
                    name
                )
                VALUES ($1, $2, $3);
            "#,
            cultivar.id.0,
            cultivar.category_id.0,
            cultivar.name
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Cultivar inserted successfully: {:?}", result);
                Ok(cultivar.id)
            }
            Err(err) => {
                // Handle database constraint error
                handle_cultivar_database_error(&err)?;

                tracing::error!("Database error, failed to insert cultivar: {}", err);
                Err(err.into())
            }
        }
    }

    /// Updates cultivar in the database
    #[tracing::instrument(name = "Update Cultivar", skip(db, cultivar))]
    pub async fn update(
        id: ModelID,
        cultivar: CultivarUpdateData,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE services.cultivars cultivar
                SET name = COALESCE($1, cultivar.name),
                    category_id = COALESCE($2, cultivar.category_id)
                WHERE cultivar.id = $3
           "#,
            cultivar.name,
            cultivar.category_id.0,
            id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Cultivar updated successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_cultivar_database_error(&err)?;

                tracing::error!("Database error, failed to update cultivar: {}", err);
                Err(err.into())
            }
        }
    }

    /// Deletes cultivar from the database
    #[tracing::instrument(name = "Delete Cultivar", skip(db))]
    pub async fn delete(id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                DELETE FROM services.cultivars cultivar
                WHERE cultivar.id = $1
                RETURNING cultivar.image
           "#,
            id.0
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(record) => {
                if let Some(path) = record.image {
                    if delete_cultivar_photo(&path).await.is_err() {
                        tracing::error!("Io error, failed to delete cultivar image-path: {path}, but Cultivar was deleted successfully.");
                        return Ok(());
                    }
                }
                tracing::debug!("Cultivar deleted successfully");
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_cultivar_database_error(&err)?;

                tracing::error!("Database error, failed to delete cultivar: {}", err);
                Err(err.into())
            }
        }
    }

    /// Insert cultivar image-path into the database
    ///
    /// Returning new and old images paths
    #[tracing::instrument(name = "Database::cultivar-insert-image", skip(db))]
    pub async fn insert_photo(
        id: ModelID,
        paths: Vec<PathBuf>,
        db: DatabaseConnection,
    ) -> ServerResult<(String, Option<String>)> {
        let path = files::get_jpg_path(paths)?;
        match sqlx::query!(
            r#"
                UPDATE services.cultivars cultivar
                SET image = $1
                WHERE cultivar.id = $2

                RETURNING (
                    SELECT cultivar.image
                    FROM services.cultivars cultivar
                    WHERE  cultivar.id = $2
                ) AS old_image
           "#,
            path,
            id.0
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(rec) => {
                tracing::debug!("Cultivar image inserted successfully");
                Ok((path, rec.old_image))
            }
            Err(err) => {
                // Handle database constraint error
                handle_cultivar_database_error(&err)?;

                tracing::error!(
                    "Database error, failed to insert cultivar image-path: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Delete cultivar image path from the database
    #[tracing::instrument(name = "Database::cultivar-delete-image", skip(db))]
    pub async fn delete_photo(id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE services.cultivars cultivar
                SET image = NULL
                WHERE cultivar.id = $1

                RETURNING (
                    SELECT cultivar.image
                    FROM services.cultivars cultivar
                    WHERE  cultivar.id = $1
                ) AS image
           "#,
            id.0
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(rec) => {
                tracing::debug!("Cultivar image-path deleted successfully");

                // Delete images from the file system
                if let Some(image) = rec.image {
                    tokio::spawn(async move { delete_cultivar_photo(&image).await });
                }

                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_cultivar_database_error(&err)?;

                tracing::error!(
                    "Database error, failed to delete cultivar image-path: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Fetches cultivars identifies from the database
    pub async fn index(db: DatabaseConnection) -> ServerResult<ModelIndex> {
        match sqlx::query!(
            r#"
                SELECT cultivar.id,
                    cultivar.name
                FROM services.cultivars cultivar
            "#
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let cultivar_index = records
                    .into_iter()
                    .map(|rec| ModelIdentifier::from_row(rec.id.into(), rec.name))
                    .collect();

                Ok(cultivar_index)
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to fetch cultivar identifiers: {}",
                    err
                );
                Err(err.into())
            }
        }
    }
}

/// Handle cultivar database constraints errors
// #[allow(clippy::cognitive_complexity)]
fn handle_cultivar_database_error(err: &sqlx::Error) -> ServerResult<()> {
    if let sqlx::Error::Database(db_err) = err {
        // Handle db unique constraints
        if db_err.is_unique_violation() {
            tracing::error!("Database error, cultivar already exists. {:?}", err);
            return Err(ServerError::rejection(EndpointRejection::Conflict(
                "Cultivar already exists.".into(),
            )));
        }
        // Handle db foreign key constraints
        if db_err.is_foreign_key_violation() {
            tracing::error!("Database error, cultivar category not found. {:?}", err);
            return Err(ServerError::rejection(EndpointRejection::BadRequest(
                "Cultivar category not found.".into(),
            )));
        }
    }

    // For updates only
    if matches!(err, &sqlx::Error::RowNotFound) {
        tracing::error!("Database error, cultivar not found. {:?}", err);
        return Err(ServerError::rejection(EndpointRejection::NotFound(
            "Cultivar not found.".into(),
        )));
    }

    Ok(())
}
