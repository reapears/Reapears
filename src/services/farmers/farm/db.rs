//! Farm database impl

use std::path::PathBuf;

use itertools::Itertools;
use time::OffsetDateTime;

use crate::{
    endpoint::EndpointRejection,
    error::{ServerError, ServerResult},
    files,
    server::state::DatabaseConnection,
    services::{
        farmers::location::models::{Location, LocationIndex},
        produce::harvest::{delete_harvest_photos, models::HarvestIndex},
    },
    types::ModelID,
    types::{ModelIdentifier, ModelIndex, Pagination},
};

use super::{
    forms::{FarmInsertData, FarmUpdateData},
    models::{Farm, FarmIndex, FarmList},
    utils::{
        archive_farm, archive_farm_harvests, archive_farm_locations, delete_farm,
        delete_farm_harvests, delete_farm_locations, farm_archived_harvest_count,
        farm_harvest_images, location_insert, update_user_is_farmer, user_farm_count,
    },
};

impl Farm {
    /// Fetches farm records from the database
    #[tracing::instrument(name = "Fetch FarmList", skip(db))]
    pub async fn records(pg: Pagination, db: DatabaseConnection) -> ServerResult<FarmList> {
        let (offset, limit) = pg.offset_limit();
        match sqlx::query!(
            r#"
                WITH locations_metadata AS(
                    SELECT location_.id AS location_id,
                        COUNT(harvest.id) AS harvests_count
                        FROM services.active_locations location_
                        LEFT JOIN services.active_harvests harvest
                            ON location_.id = harvest.location_id
                    GROUP BY location_.id
                )
                SELECT farm.id AS "farm_id!",
                    farm.owner_id AS "farm_owner_id!",
                    farm.name AS "farm_name!",
                    farm.logo AS "farm_logo",
                    user_.first_name AS "farm_owner_first_name!",
                    user_.last_name AS farm_owner_last_name,
                    profile.photo AS farm_owner_photo,
                    location_.id AS "location_id!",
                    location_.place_name AS "location_place_name!",
                    location_.coords AS location_coords,
                    region.name AS location_region,
                    country.name AS "location_country!",
                    location_md.harvests_count
                FROM services.active_farms farm
                LEFT JOIN accounts.users user_
                    ON farm.owner_id = user_.id
                LEFT JOIN accounts.user_profiles profile
                    ON user_.id = profile.user_id
                LEFT JOIN services.active_locations location_
                    ON farm.id = location_.farm_id
                LEFT JOIN locations_metadata location_md
                    ON location_.id = location_md.location_id
                LEFT JOIN services.countries country
                    ON location_.country_id = country.id
                LEFT JOIN services.regions region
                   ON location_.region_id = region.id

                --ORDER BY farm.name
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
                let mut farms = Vec::new();
                for (farm_id, farm_group) in &records.into_iter().group_by(|rec| rec.farm_id) {
                    let farm_group: Vec<_> = farm_group.collect();
                    let first_rec = &farm_group[0];

                    let farm_name = first_rec.farm_name.clone();
                    let farm_logo = first_rec.farm_logo.clone();

                    let owner_id = first_rec.farm_owner_id.into();
                    let owner_first_name = first_rec.farm_owner_first_name.clone();
                    let owner_last_name = first_rec.farm_owner_last_name.clone();
                    let owner_photo = first_rec.farm_owner_photo.clone();

                    let locations = farm_group
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

                    farms.push(FarmIndex::from_row(
                        farm_id.into(),
                        farm_name,
                        farm_logo,
                        locations,
                        owner_id,
                        owner_first_name,
                        owner_last_name,
                        owner_photo,
                    ));
                }

                Ok(farms)
            }
            Err(err) => {
                tracing::error!("Database error, failed to fetch farms: {}", err);
                Err(err.into())
            }
        }
    }

    /// Fetches farm detail from the database
    #[tracing::instrument(name = "Find Farm", skip(db))]
    pub async fn find(id: ModelID, db: DatabaseConnection) -> ServerResult<Option<Self>> {
        match sqlx::query!(
            r#"
                SELECT farm.id AS "farm_id!",
                    farm.owner_id as "farm_owner_id!",
                    farm.name AS "farm_name!",
                    farm.logo AS "farm_logo",
                    farm.contact_email AS "farm_contact_email",
                    farm.contact_number AS "farm_contact_number",
                    farm.registered_on AS "farm_registered_on!",
                    user_.first_name AS farm_owner_first_name,
                    user_.last_name AS farm_owner_last_name,
                    profile.photo AS farm_owner_photo,
                    location_.id AS "location_id!",
                    location_.place_name AS "location_place_name!",
                    location_.coords AS location_coords,
                    location_.description AS location_description,
                    country.name AS location_country,
                    region.name AS "location_region?",
                    harvest.id AS "harvest_id?",
                    harvest.price AS "harvest_price?",
                    harvest.images AS harvest_images,
                    harvest.harvest_date AS "harvest_harvest_date?",
                    cultivar.name AS "cultivar_name?",
                    cultivar_category.name AS "cultivar_category?",
                    cultivar.image AS cultivar_image
                FROM services.active_farms farm
                LEFT JOIN accounts.users user_
                    ON farm.owner_id = user_.id
                LEFT JOIN accounts.user_profiles profile
                    ON user_.id = profile.user_id
                LEFT JOIN services.active_locations location_
                    ON farm.id = location_.farm_id
                LEFT JOIN services.countries country
                    ON location_.country_id = country.id
                LEFT JOIN services.regions region
                    ON location_.region_id = region.id
                LEFT JOIN services.active_harvests harvest
                    ON location_.id = harvest.location_id
                LEFT JOIN services.cultivars cultivar
                    ON harvest.cultivar_id = cultivar.id
                LEFT JOIN services.cultivar_categories cultivar_category
                    ON cultivar.category_id = cultivar_category.id

                WHERE farm.id = $1
            "#,
            id.0,
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) if records.is_empty() => Ok(None),
            Ok(records) => {
                let first_rec = &records[0];

                let farm_id = first_rec.farm_id.into();
                let farm_name = first_rec.farm_name.clone();
                let farm_logo = first_rec.farm_logo.clone();
                let farm_contact_email = first_rec.farm_contact_email.clone();
                let farm_contact_number = first_rec.farm_contact_number.clone();

                let registered_on = first_rec.farm_registered_on;
                let owner_id = first_rec.farm_owner_id.into();
                let owner_first_name = first_rec.farm_owner_first_name.clone();
                let owner_last_name = first_rec.farm_owner_last_name.clone();
                let owner_photo = first_rec.farm_owner_photo.clone();

                let mut locations = Vec::new();

                for (location_id, location_group) in
                    &records.into_iter().group_by(|rec| rec.location_id)
                {
                    let location_group: Vec<_> = location_group.collect();
                    let first_rec = &location_group[0];

                    let place_name = first_rec.location_place_name.clone();
                    let region = first_rec.location_region.clone();
                    let country = first_rec.location_country.clone();
                    let coords = first_rec.location_coords.clone();
                    let description = first_rec.location_description.clone();
                    let farm_name = first_rec.farm_name.clone();

                    // Create harvests available at the location
                    let harvests: Vec<_> = location_group
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

                    locations.push(Location::from_row(
                        location_id.into(),
                        place_name,
                        region,
                        country,
                        coords,
                        description,
                        farm_id,
                        farm_name,
                        harvests,
                    ));
                }

                let farm = Self::from_row(
                    farm_id,
                    farm_name,
                    farm_logo,
                    farm_contact_email,
                    farm_contact_number,
                    locations,
                    registered_on,
                    owner_id,
                    owner_first_name,
                    owner_last_name,
                    owner_photo,
                );
                Ok(Some(farm))
            }
            Err(err) => {
                tracing::error!("Database error, failed to fetch farm: {}", err);
                Err(err.into())
            }
        }
    }

    /// Inserts farm into the database
    #[tracing::instrument(name = "Insert Farm", skip(db, farm))]
    pub async fn insert(farm: FarmInsertData, db: DatabaseConnection) -> ServerResult<ModelID> {
        let mut tx = db.pool.begin().await?; // init transaction
        match sqlx::query!(
            r#"
                INSERT INTO services.farms(
                    id,
                    owner_id,
                    name,
                    contact_number,
                    contact_email,
                    founded_at,
                    registered_on,
                    deleted
                )
                VALUES($1, $2, $3, $4, $5, $6, $7, false);
            "#,
            farm.id.0,
            farm.owner_id.0,
            farm.name,
            farm.contact_number,
            farm.contact_email,
            farm.founded_at,
            farm.registered_on,
        )
        .execute(&mut *tx)
        .await
        {
            Ok(result) => {
                tracing::debug!(
                    "Farm insert successfully, but transaction not committed: {:?}",
                    result
                );
                // Insert farm location
                location_insert(farm.location, &mut tx).await?;

                //Update user
                update_user_is_farmer(true, farm.owner_id, &mut tx).await?;

                tx.commit().await?; // Commit transaction
                tracing::debug!("Farm and its location inserted successfully.");
                Ok(farm.id)
            }

            Err(err) => {
                // Handle database constraint error
                handle_farm_database_error(&err)?;

                tracing::error!("Database error, failed to insert farm: {}", err);
                Err(err.into())
            }
        }
    }

    /// Updates farm in the database
    #[tracing::instrument(name = "Update Farm", skip(db, farm))]
    pub async fn update(
        id: ModelID,
        farm: FarmUpdateData,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE services.farms farm
                SET name = COALESCE($1, farm.name),
                    contact_number = $2,
                    contact_email = $3,
                    founded_at = $4

                WHERE id = $5;
           "#,
            farm.name,
            farm.contact_number,
            farm.contact_email,
            farm.founded_at,
            id.0,
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Farm updated successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_farm_database_error(&err)?;

                tracing::error!("Database error, failed to update farm: {}", err);
                Err(err.into())
            }
        }
    }

    /// Insert farm logo path into the database
    #[tracing::instrument(skip(db), name = "Insert farm logo")]
    pub async fn insert_or_delete_logo(
        id: ModelID,
        paths: Option<Vec<PathBuf>>,
        db: DatabaseConnection,
    ) -> ServerResult<(Option<String>, Option<String>)> {
        let path = match paths {
            Some(paths) => Some(files::get_jpg_path(paths)?),
            None => None,
        };

        match sqlx::query!(
            r#"
                 UPDATE services.farms farm
                 SET logo = $1
                 WHERE farm.id = $2
 
                 RETURNING (
                     SELECT farm.logo
                     FROM services.farms farm
                     WHERE farm.id = $2
                 ) AS old_logo
            "#,
            path,
            id.0
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(rec) => {
                tracing::debug!("Farm logo inserted successfully");
                Ok((path, rec.old_logo))
            }
            Err(err) => {
                // Handle database constraint error
                handle_farm_database_error(&err)?;

                tracing::error!("Database error, failed to set farm logo: {}", err);
                Err(err.into())
            }
        }
    }

    /// Deletes farm from the database
    ///
    // Farm will only be `deleted` if it has no location
    // that has harvests associated with it.
    #[tracing::instrument(name = "Delete Farm", skip(db))]
    #[allow(clippy::cast_sign_loss)]
    pub async fn delete(id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        let conn = db.clone();
        let old_archived_harvest_count =
            tokio::spawn(async move { farm_archived_harvest_count(id, conn).await }).await??;

        // Fetch farm active harvests images
        let conn = db.clone();
        let image_paths =
            tokio::spawn(async move { farm_harvest_images(id, conn).await }).await??;

        // Fetch farm count belonging to the user of the current farm
        let conn = db.clone();
        let (user_id, farm_count) =
            tokio::spawn(async move { user_farm_count(id, conn).await }).await??;
        let user_id = user_id
            .ok_or_else(|| ServerError::new("Database error, failed to fetch farm owner."))?;

        // initialize transaction
        let mut tx = db.pool.begin().await?;

        let deleted_at = OffsetDateTime::now_utc();

        // Cleanup harvests
        let new_archived_harvest_count = archive_farm_harvests(id, deleted_at, &mut tx).await?;
        delete_farm_harvests(id, deleted_at, &mut tx).await?;

        // Cleanup locations
        archive_farm_locations(id, deleted_at, &mut tx).await?;
        delete_farm_locations(id, &mut tx).await?;

        let archived_count = old_archived_harvest_count as u64 + new_archived_harvest_count;

        // Delete or archive farm
        if archived_count == 0 {
            delete_farm(id, &mut tx).await?;
        } else {
            archive_farm(id, deleted_at, &mut tx).await?;
        }

        // If this was the only user's farm, set user is no longer a farmer
        if farm_count == 1 {
            update_user_is_farmer(false, user_id, &mut tx).await?;
        }

        tx.commit().await?;
        tracing::debug!("Farm::delete, transaction committed successfully.");

        // Delete active harvest images
        tokio::spawn(async move { delete_harvest_photos(image_paths.into_iter().flatten()).await });

        Ok(())
    }

    /// Fetches farm's location identifiers from the database
    pub async fn location_index(
        farm_id: ModelID,
        db: DatabaseConnection,
    ) -> ServerResult<ModelIndex> {
        match sqlx::query!(
            r#"
                SELECT location_.id AS "id!",
                    location_.place_name AS "place_name!"
                FROM services.active_locations location_

                WHERE location_.farm_id = $1
            "#,
            farm_id.0
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let locations = records
                    .into_iter()
                    .map(|rec| ModelIdentifier::from_row(rec.id.into(), rec.place_name))
                    .collect();

                Ok(locations)
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to fetch farm location index: {}",
                    err
                );
                Err(err.into())
            }
        }
    }
}

/// Handle farms database constraints errors
#[allow(clippy::cognitive_complexity)]
pub fn handle_farm_database_error(err: &sqlx::Error) -> ServerResult<()> {
    if let sqlx::Error::Database(db_err) = err {
        // Handle db foreign key constraints
        if db_err.is_foreign_key_violation() {
            tracing::error!("Database error, user not found. {:?}", err);
            return Err(ServerError::rejection(EndpointRejection::BadRequest(
                "User not found.".into(),
            )));
        }
    }

    // For updates only
    if matches!(err, &sqlx::Error::RowNotFound) {
        tracing::error!("Database error, farm not found. {:?}", err);
        return Err(ServerError::rejection(EndpointRejection::NotFound(
            "Farm not found.".into(),
        )));
    }

    Ok(())
}
