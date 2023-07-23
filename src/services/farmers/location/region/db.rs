//! Cultivar category database impl

use crate::{
    endpoint::EndpointRejection,
    error::{ServerError, ServerResult},
    server::state::DatabaseConnection,
    types::ModelID,
};

use super::{
    forms::{RegionInsertData, RegionUpdateData},
    Region, RegionList,
};

impl Region {
    /// Fetches location region records from the database
    #[tracing::instrument(name = "Database::records-location-region", skip(db))]
    pub async fn records(db: DatabaseConnection) -> ServerResult<RegionList> {
        match sqlx::query!(
            r#"
                SELECT  region.id,
                     region.name
                FROM services.regions region
            "#
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let regions = records
                    .into_iter()
                    .map(|rec| Self::from_row(rec.id.into(), rec.name))
                    .collect();

                Ok(regions)
            }
            Err(err) => {
                tracing::error!("Database error, failed to fetch location regions: {}", err);
                Err(err.into())
            }
        }
    }

    /// Inserts location region into the database
    #[tracing::instrument(name = "Insert Location-region", skip(db, region))]
    pub async fn insert(region: RegionInsertData, db: DatabaseConnection) -> ServerResult<ModelID> {
        match sqlx::query!(
            r#"
                INSERT INTO services.regions (
                    id,
                    country_id,
                    name
                )
                VALUES ($1, $2, $3);
            "#,
            region.id.0,
            region.country_id.0,
            region.name
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Location region inserted successfully: {:?}", result);
                Ok(region.id)
            }
            Err(err) => {
                // Handle database constraint error
                handle_region_database_error(&err)?;

                tracing::error!("Database error, failed to insert location region: {}", err);
                Err(err.into())
            }
        }
    }

    /// Updates location region in the database
    #[tracing::instrument(name = "Update Location-region", skip(db, region))]
    pub async fn update(
        id: ModelID,
        region: RegionUpdateData,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE services.regions region
                SET name = COALESCE($1, region.name),
                    country_id = COALESCE($2, region.country_id)
                WHERE region.id = $3
           "#,
            region.name,
            region.country_id.map(|id| id.0),
            id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Location region updated successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_region_database_error(&err)?;

                tracing::error!("Database error, failed to update location region: {}", err);
                Err(err.into())
            }
        }
    }

    // /// Deletes location region from the database
    #[tracing::instrument(name = "Delete Location-region", skip(db))]
    pub async fn delete(id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                DELETE FROM services.regions region
                WHERE region.id = $1
           "#,
            id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Location region deleted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_region_database_error(&err)?;

                tracing::error!("Database error, failed to delete location region: {}", err);
                Err(err.into())
            }
        }
    }
}

/// Handle regions database constraints errors
#[allow(clippy::cognitive_complexity)]
fn handle_region_database_error(err: &sqlx::Error) -> ServerResult<()> {
    if let sqlx::Error::Database(db_err) = err {
        // Handle db unique constraints
        if db_err.is_unique_violation() {
            tracing::error!("Database error, Region already exists. {:?}", err);
            return Err(ServerError::rejection(EndpointRejection::Conflict(
                "Region already exists.".into(),
            )));
        }

        // Handle db foreign key constraints
        if db_err.is_foreign_key_violation() {
            tracing::error!("Database error, country not found. {:?}", err);
            return Err(ServerError::rejection(EndpointRejection::BadRequest(
                "Country  not found.".into(),
            )));
        }
    }

    // For updates only
    if matches!(err, &sqlx::Error::RowNotFound) {
        tracing::error!("Database error, region not found. {:?}", err);
        return Err(ServerError::rejection(EndpointRejection::NotFound(
            "Region not found.".into(),
        )));
    }

    Ok(())
}
