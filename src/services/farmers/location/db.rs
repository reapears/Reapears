//! Location database impl

use time::OffsetDateTime;

use crate::{
    endpoint::EndpointRejection,
    error::{ServerError, ServerResult},
    server::state::DatabaseConnection,
    services::produce::harvest::{delete_harvest_photos, models::HarvestIndex},
    types::ModelID,
    types::{ModelIdentifier, ModelIndex, Pagination},
};

use super::{
    forms::{LocationInsertData, LocationUpdateData},
    models::{Location, LocationIndex, LocationList},
    utils::{
        archive_location, archive_location_harvests, delete_location, delete_location_harvests,
        location_archived_harvest_count, location_harvest_photos,
    },
};

impl Location {
    /// Fetches farm location records from the database
    #[tracing::instrument(name = "Fetch LocationList", skip(db))]
    pub async fn records(pg: Pagination, db: DatabaseConnection) -> ServerResult<LocationList> {
        let (offset, limit) = pg.offset_limit();
        match sqlx::query!(
            r#"
                SELECT location_.id AS "location_id!",
                    location_.place_name AS "location_place_name!",
                    location_.coords AS location_coords,
                    region.name AS "location_region?",
                    country.name AS location_country,
                    farm.name AS farm_name,
                    (SELECT count(harvest.id)
                     FROM services.active_harvests harvest
                     WHERE location_.id = harvest.location_id) AS harvests_count
                FROM services.active_locations location_
                LEFT JOIN services.farms farm
                    ON location_.farm_id = farm.id
                LEFT JOIN services.regions region
                    ON location_.region_id = region.id
                LEFT JOIN services.countries country
                    ON location_.country_id = country.id

                ORDER BY location_.place_name
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
                let locations = records
                    .into_iter()
                    .map(|rec| {
                        LocationIndex::from_row(
                            rec.location_id.into(),
                            rec.location_place_name,
                            rec.location_region,
                            rec.location_country,
                            rec.location_coords,
                            rec.farm_name,
                            rec.harvests_count,
                        )
                    })
                    .collect();

                Ok(locations)
            }
            Err(err) => {
                tracing::error!("Database error, failed to fetch locations: {}", err);
                Err(err.into())
            }
        }
    }

    /// Fetches farm location detail from the database
    #[tracing::instrument(name = "Find Location", skip(db))]
    pub async fn find(
        id: ModelID,
        pg: Option<Pagination>,
        db: DatabaseConnection,
    ) -> ServerResult<Option<Self>> {
        let (offset, limit) = pg.unwrap_or_default().offset_limit();
        match sqlx::query!(
            r#"
                SELECT location_.id AS location_id,
                    location_.place_name AS location_place_name,
                    location_.coords AS location_coords,
                    location_.description AS location_description,
                    region.name AS "location_region?",
                    country.name AS location_country,
                    farm.id AS farm_id,
                    farm.name AS farm_name,
                    farm.logo AS farm_logo,
                    harvest.id AS "harvest_id?",
                    harvest.price AS "harvest_price?",
                    harvest.harvest_date AS "harvest_harvest_date?",
                    harvest.images AS harvest_images,
                    cultivar.name AS "cultivar_name?",
                    cultivar_category.name AS "cultivar_category?",
                    cultivar.image AS cultivar_image
                FROM services.locations location_
                LEFT JOIN services.farms farm
                    ON location_.farm_id = farm.id
                LEFT JOIN services.regions region
                    ON location_.region_id = region.id
                LEFT JOIN services.countries country
                    ON location_.country_id = country.id
                LEFT JOIN services.active_harvests harvest
                    ON location_.id = harvest.location_id
                LEFT JOIN services.cultivars cultivar
                    ON harvest.cultivar_id = cultivar.id
                LEFT JOIN services.cultivar_categories cultivar_category
                    ON cultivar.category_id = cultivar_category.id

                WHERE location_.id = $1
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

                let location_id = first_rec.location_id.into();
                let farm_id = first_rec.farm_id.into();
                let farm_name = first_rec.farm_name.clone();
                let place_name = first_rec.location_place_name.clone();
                let region = first_rec.location_region.clone();
                let country = first_rec.location_country.clone();
                let coords = first_rec.location_coords.clone();
                let description = first_rec.location_description.clone();

                let harvests: Vec<_> = records
                    .into_iter()
                    .filter(|rec| rec.harvest_id.is_some())
                    .map(|rec| {
                        HarvestIndex::from_row(
                            rec.harvest_id.unwrap().into(),
                            rec.harvest_price.unwrap(),
                            rec.harvest_harvest_date.unwrap(),
                            rec.harvest_images,
                            rec.cultivar_name.unwrap(),
                            rec.cultivar_category.unwrap(),
                            rec.cultivar_image,
                            rec.location_place_name,
                            rec.location_region,
                            rec.location_country,
                            rec.location_coords,
                            rec.farm_name,
                            rec.farm_logo,
                            0.into(), // boost amount not important
                        )
                    })
                    .collect();

                let harvests = (!harvests.is_empty()).then_some(harvests);

                let location = Self::from_row(
                    location_id,
                    place_name,
                    region,
                    country,
                    coords,
                    description,
                    farm_id,
                    farm_name,
                    harvests,
                );

                Ok(Some(location))
            }
            Err(err) => {
                tracing::error!("Database error, failed to fetch location: {}", err);
                Err(err.into())
            }
        }
    }

    /// Inserts farm location in the database
    #[tracing::instrument(name = "Insert Location", skip(db, location))]
    pub async fn insert(
        location: LocationInsertData,
        db: DatabaseConnection,
    ) -> ServerResult<ModelID> {
        match sqlx::query!(
            r#"
                INSERT INTO services.locations(
                    id, 
                    farm_id, 
                    place_name, 
                    region_id, 
                    country_id, 
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
            location.region_id.0,
            location.country_id.0,
            location.description,
            location.coords,
            location.created_at,
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Location inserted successfully: {:?}", result);
                Ok(location.id)
            }
            Err(err) => {
                // Handle database constraint error
                handle_location_database_error(&err)?;

                tracing::error!("Database error, failed to insert location: {}", err);
                Err(err.into())
            }
        }
    }

    /// Updates farm location in the database
    #[tracing::instrument(name = "Update Location", skip(db, location))]
    pub async fn update(
        id: ModelID,
        location: LocationUpdateData,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE services.locations location
                SET place_name = COALESCE($1, location.place_name),
                    region_id = $2,
                    country_id = COALESCE($3, location.country_id),
                    description = COALESCE($4, location.description),
                    coords = $5
                WHERE location.id = $6;
            "#,
            location.place_name,
            location.region_id.0,
            location.country_id.0,
            location.description,
            location.coords,
            id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Location updated successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_location_database_error(&err)?;

                tracing::error!("Database error, failed to update location:  {}", err);
                Err(err.into())
            }
        }
    }

    /// Deletes farm location from the database
    ///
    /// Location will ony be deleted if its not the only farm location
    /// and it does not have any harvests incl archived ones
    #[tracing::instrument(name = "Delete Location", skip(db))]
    #[allow(clippy::cast_sign_loss)]
    pub async fn delete(id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        let image_paths = tokio::spawn({
            let db = db.clone();
            async move { location_harvest_photos(id, db).await }
        })
        .await??;

        let old_archived_harvest_count = tokio::spawn({
            let db = db.clone();
            async move { location_archived_harvest_count(id, db).await }
        })
        .await??;

        let mut tx = db.pool.begin().await?;

        // Cleanup harvests
        let finished_at = OffsetDateTime::now_utc();
        let new_archived_harvest_count =
            archive_location_harvests(id, finished_at, &mut tx).await?;
        _ = delete_location_harvests(id, finished_at, &mut tx).await?;

        let archived_count = old_archived_harvest_count as u64 + new_archived_harvest_count;

        // Delete or archive location
        if archived_count == 0 {
            delete_location(id, &mut tx).await?;
        } else {
            archive_location(id, &mut tx).await?;
        }

        tx.commit().await?;
        tracing::debug!("Location::delete transaction committed successfully.");

        // Delete active harvest images
        tokio::spawn(async move { delete_harvest_photos(image_paths.into_iter().flatten()).await });

        Ok(())
    }

    /// Fetches country regions from the database
    pub async fn regions(country_id: ModelID, db: DatabaseConnection) -> ServerResult<ModelIndex> {
        match sqlx::query!(
            r#"
                SELECT region.id,
                    region.name
                FROM services.regions region

                WHERE region.country_id = $1
            "#,
            country_id.0
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let regions = records
                    .into_iter()
                    .map(|rec| ModelIdentifier::from_row(rec.id.into(), rec.name))
                    .collect();

                Ok(regions)
            }
            Err(err) => {
                tracing::error!("Database error, failed to fetch regions: {}", err);
                Err(err.into())
            }
        }
    }

    /// Fetches country from the database
    pub async fn countries(db: DatabaseConnection) -> ServerResult<ModelIndex> {
        match sqlx::query!(
            r#"
                SELECT country.id,
                    country.name
                FROM services.countries country
            "#,
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let countries = records
                    .into_iter()
                    .map(|rec| ModelIdentifier::from_row(rec.id.into(), rec.name))
                    .collect();

                Ok(countries)
            }
            Err(err) => {
                tracing::error!("Database error, failed to fetch countries: {}", err);
                Err(err.into())
            }
        }
    }
}

/// Handle locations database constraints errors
#[allow(clippy::cognitive_complexity)]
pub fn handle_location_database_error(err: &sqlx::Error) -> ServerResult<()> {
    if let sqlx::Error::Database(db_err) = err {
        // Handle db foreign key constraints
        if db_err.is_foreign_key_violation() {
            if let Some(constraint) = db_err.constraint() {
                if constraint == "locations_farm_id_fkey" {
                    tracing::error!("Database error, farm not found. {:?}", err);
                    return Err(ServerError::rejection(EndpointRejection::BadRequest(
                        "Farm not found.".into(),
                    )));
                }

                if constraint == "locations_country_id_fkey" {
                    tracing::error!("Database error, country not found. {:?}", err);
                    return Err(ServerError::rejection(EndpointRejection::BadRequest(
                        "Country not found.".into(),
                    )));
                }

                if constraint == "locations_region_id_fkey" {
                    tracing::error!("Database error, region not found. {:?}", err);
                    return Err(ServerError::rejection(EndpointRejection::BadRequest(
                        "Region not found.".into(),
                    )));
                }
            }
            tracing::error!(
                "Database error, farm or country or region not found. {:?}",
                err
            );
            return Err(ServerError::rejection(EndpointRejection::BadRequest(
                "farm or country or region  not found.".into(),
            )));
        }
    }

    // For updates only
    if matches!(err, &sqlx::Error::RowNotFound) {
        tracing::error!("Database error, location not found. {:?}", err);
        return Err(ServerError::rejection(EndpointRejection::NotFound(
            "Location not found.".into(),
        )));
    }

    Ok(())
}
