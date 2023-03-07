//! Cultivar category database impl

use uuid::Uuid;

use crate::{error::ServerResult, server::state::DatabaseConnection};

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
                    .map(|rec| Self::from_row(rec.id, rec.name))
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
    pub async fn insert(region: RegionInsertData, db: DatabaseConnection) -> ServerResult<Uuid> {
        match sqlx::query!(
            r#"
                INSERT INTO services.regions (
                    id,
                    country_id,
                    name
                )
                VALUES ($1, $2, $3);
            "#,
            region.id,
            region.country_id,
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
                tracing::error!("Database error, failed to insert location region: {}", err);
                Err(err.into())
            }
        }
    }

    /// Updates location region in the database
    #[tracing::instrument(name = "Update Location-region", skip(db, region))]
    pub async fn update(
        id: Uuid,
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
            region.country_id,
            id
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Location region updated successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!("Database error, failed to update location region: {}", err);
                Err(err.into())
            }
        }
    }

    // /// Deletes location region from the database
    #[tracing::instrument(name = "Delete Location-region", skip(db))]
    pub async fn delete(id: Uuid, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                DELETE FROM services.regions region
                WHERE region.id = $1
           "#,
            id
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Location region deleted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!("Database error, failed to delete location region: {}", err);
                Err(err.into())
            }
        }
    }
}
