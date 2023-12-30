//! Harvest database impl

use futures_util::stream::{Stream, TryStreamExt};

use crate::{
    endpoint::EndpointRejection,
    error::{ServerError, ServerResult},
    server::state::DatabaseConnection,
    types::ModelID,
    types::Pagination,
};

use super::{
    forms::{HarvestInsertData, HarvestUpdateData},
    models::{Harvest, HarvestIndex, HarvestList},
    utils::{delete_harvest_photos, delete_or_archive_harvest, find_delete_harvest},
};

impl Harvest {
    /// Fetches a stream of harvest records from the database
    #[tracing::instrument(name = "Fetch HarvestStream", skip(db))]
    pub async fn stream<'a>(
        db: &'a DatabaseConnection,
    ) -> impl Stream<Item = Result<HarvestIndex, sqlx::Error>> + 'a {
        //NB! Don't forget to select harvests from services.active_harvests
        let today = time::OffsetDateTime::now_utc().date();
        sqlx::query!(
            r#"
                SELECT harvest.id AS "harvest_id!",
                    harvest.cultivar_id,
                    harvest.price AS "harvest_price!",
                    harvest.harvest_date AS "harvest_harvest_date!",
                    harvest.images AS harvest_images,
                    cultivar.name AS cultivar_name,
                    cultivar_category.name AS cultivar_category,
                    cultivar.image AS cultivar_image, 
                    farm.name AS farm_name,
                    farm.logo AS farm_logo,
                    location_.place_name AS location_place_name,
                    location_.coords AS location_coords,
                    region.name AS "location_region?",
                    country.name AS location_country,
                    subscription.amount AS "boost_amount?",
                    subscription.expires_at AS "subscription_expires_at?"
                FROM services.active_harvests harvest
                LEFT JOIN services.cultivars cultivar
                    ON harvest.cultivar_id = cultivar.id
                LEFT JOIN services.cultivar_categories cultivar_category
                    ON cultivar.category_id = cultivar_category.id
                LEFT JOIN services.locations location_
                    ON harvest.location_id = location_.id
                LEFT JOIN services.farms farm
                    ON location_.farm_id = farm.id
                LEFT JOIN services.regions region
                    ON location_.region_id = region.id
                LEFT JOIN services.countries country
                    ON location_.country_id = country.id

                LEFT JOIN features.harvest_subscriptions subscription
                    ON harvest.id  = subscription.harvest_id

                ORDER BY subscription.amount DESC NULLS LAST,
                    greatest(AGE(harvest.harvest_date), -AGE(harvest.harvest_date));
            "#,
        )
        .fetch(&db.pool)
        .map_ok(move |rec| {
            HarvestIndex::from_row(
                rec.harvest_id.into(),
                rec.harvest_price,
                rec.harvest_harvest_date,
                rec.harvest_images,
                rec.cultivar_name,
                rec.cultivar_category,
                rec.cultivar_image,
                rec.location_place_name,
                rec.location_region,
                rec.location_country,
                rec.location_coords,
                rec.farm_name,
                rec.farm_logo,
                calc_boost_amount(rec.boost_amount, rec.subscription_expires_at, today),
            )
        })
    }

    /// Fetches harvest records from the database
    #[tracing::instrument(name = "Fetch HarvestList", skip(db))]
    pub async fn records(pg: Pagination, db: DatabaseConnection) -> ServerResult<HarvestList> {
        //NB! Don't forget to select harvests from services.active_harvests
        let (offset, limit) = pg.offset_limit();
        match sqlx::query!(
            r#"
                SELECT harvest.id AS "harvest_id!",
                    harvest.cultivar_id,
                    harvest.price AS "harvest_price!",
                    harvest.harvest_date AS "harvest_harvest_date!",
                    harvest.images AS harvest_images,
                    cultivar.name AS cultivar_name,
                    cultivar_category.name AS cultivar_category,
                    cultivar.image AS cultivar_image, 
                    farm.name AS farm_name,
                    farm.logo AS farm_logo,
                    location_.place_name AS location_place_name,
                    location_.coords AS location_coords,
                    region.name AS "location_region?",
                    country.name AS location_country,
                    subscription.amount AS "boost_amount?",
                    subscription.expires_at AS "subscription_expires_at?"
                FROM services.active_harvests harvest
                LEFT JOIN services.cultivars cultivar
                    ON harvest.cultivar_id = cultivar.id
                LEFT JOIN services.cultivar_categories cultivar_category
                    ON cultivar.category_id = cultivar_category.id
                LEFT JOIN services.locations location_
                    ON harvest.location_id = location_.id
                LEFT JOIN services.farms farm
                    ON location_.farm_id = farm.id
                LEFT JOIN services.regions region
                    ON location_.region_id = region.id
                LEFT JOIN services.countries country
                    ON location_.country_id = country.id

                LEFT JOIN features.harvest_subscriptions subscription
                    ON harvest.id  = subscription.harvest_id

                ORDER BY harvest.created_at
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
                let today = time::OffsetDateTime::now_utc().date();
                let harvests = records
                    .into_iter()
                    .map(|rec| {
                        HarvestIndex::from_row(
                            rec.harvest_id.into(),
                            rec.harvest_price,
                            rec.harvest_harvest_date,
                            rec.harvest_images,
                            rec.cultivar_name,
                            rec.cultivar_category,
                            rec.cultivar_image,
                            rec.location_place_name,
                            rec.location_region,
                            rec.location_country,
                            rec.location_coords,
                            rec.farm_name,
                            rec.farm_logo,
                            calc_boost_amount(rec.boost_amount, rec.subscription_expires_at, today),
                        )
                    })
                    .collect();

                Ok(harvests)
            }

            Err(err) => {
                tracing::error!("Database error, failed to fetch harvests: {}", err);
                Err(err.into())
            }
        }
    }

    /// Fetches harvest detail from the database
    #[tracing::instrument(name = "Find Harvest", skip(db))]
    pub async fn find(id: ModelID, db: DatabaseConnection) -> ServerResult<Option<Self>> {
        //NB! Don't forget to select harvest from services.active_harvests
        match sqlx::query!(
            r#"
                SELECT harvest.id AS "harvest_id!", 
                    harvest.cultivar_id AS "cultivar_id!",
                    harvest.price AS "harvest_price!",
                    harvest.harvest_date AS "harvest_harvest_date!",
                    harvest.type AS harvest_type,
                    harvest.description AS harvest_description,
                    harvest.images AS harvest_images,
                    harvest.created_at AS "harvest_created_at!",
                    cultivar.name AS cultivar_name,
                    cultivar_category.name AS cultivar_category,
                    cultivar.image AS cultivar_image, 
                    farm.id AS farm_id,
                    farm.name AS farm_name,
                    farm.logo AS farm_logo,
                    farm.contact_number AS farm_contact_number,
                    farm.contact_email AS farm_contact_email,
                    location_.id AS location_id,
                    location_.place_name AS location_place_name,
                    location_.coords AS location_coords,
                    region.name AS "location_region?",
                    country.name AS location_country,
                    user_.id AS farm_owner_id,
                    user_.first_name AS farm_owner_first_name,
                    user_.last_name AS farm_owner_last_name,
                    profile.photo AS farm_owner_photo
                FROM services.active_harvests harvest
                LEFT JOIN services.cultivars cultivar
                    ON harvest.cultivar_id = cultivar.id
                LEFT JOIN services.cultivar_categories cultivar_category
                    ON cultivar.category_id = cultivar_category.id
                LEFT JOIN services.locations location_
                    ON harvest.location_id = location_.id
                LEFT JOIN services.farms farm
                    ON location_.farm_id = farm.id
                LEFT JOIN services.regions region
                    ON location_.region_id = region.id
                LEFT JOIN services.countries country
                    ON location_.country_id = country.id
                LEFT JOIN accounts.users user_
                    ON farm.owner_id = user_.id
                LEFT JOIN accounts.user_profiles profile
                    ON user_.id = profile.user_id 
                
                WHERE harvest.id = $1;
            "#,
            id.0
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(rec) => {
                let harvest = Self::from_row(
                    rec.harvest_id.into(),
                    rec.harvest_price,
                    rec.harvest_type,
                    rec.harvest_description,
                    rec.harvest_images,
                    rec.harvest_harvest_date,
                    rec.harvest_created_at,
                    rec.cultivar_id.into(),
                    rec.cultivar_name,
                    rec.cultivar_category,
                    rec.cultivar_image,
                    rec.location_id.into(),
                    rec.location_place_name,
                    rec.location_region,
                    rec.location_country,
                    rec.location_coords,
                    rec.farm_id.into(),
                    rec.farm_name,
                    rec.farm_logo,
                    rec.farm_contact_number,
                    rec.farm_contact_email,
                    rec.farm_owner_id.into(),
                    rec.farm_owner_first_name,
                    rec.farm_owner_last_name,
                    rec.farm_owner_photo,
                );

                Ok(Some(harvest))
            }

            Err(err) => {
                if matches!(err, sqlx::Error::RowNotFound) {
                    Ok(None)
                } else {
                    tracing::error!("Database error, failed to fetch harvest: {}", err);
                    Err(err.into())
                }
            }
        }
    }

    /// Inserts harvest in the database
    #[tracing::instrument(name = "Insert Harvest", skip(db, harvest))]
    pub async fn insert(
        harvest: HarvestInsertData,
        db: DatabaseConnection,
    ) -> ServerResult<ModelID> {
        match sqlx::query!(
            r#"
                INSERT INTO services.harvests(
                    id,
                    cultivar_id,
                    location_id, 
                    price, 
                    type, 
                    description,
                    harvest_date, 
                    created_at,
                    finished
                )
                VALUES($1, $2, $3, $4, $5, $6, $7, $8, false);
            "#,
            harvest.id.0,
            harvest.cultivar_id.0,
            harvest.location_id.0,
            harvest.price,
            harvest.r#type,
            harvest.description,
            harvest.harvest_date,
            harvest.created_at
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Harvest inserted successfully: {:?}", result);
                Ok(harvest.id)
            }
            Err(err) => {
                // Handle database constraint error
                handle_harvest_database_error(&err)?;

                tracing::error!("Database error, failed to insert harvest: {}", err);
                Err(err.into())
            }
        }
    }

    /// Updates harvest in the database
    #[tracing::instrument(name = "Update Harvest", skip(db, harvest))]
    pub async fn update(
        id: ModelID,
        harvest: HarvestUpdateData,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE services.harvests harvest
                SET cultivar_id = COALESCE($1, harvest.cultivar_id),
                    location_id = COALESCE($2, harvest.location_id),
                    price = COALESCE($3, harvest.price),
                    type = $4,
                    description = $5,
                    harvest_date = COALESCE($6, harvest.harvest_date), 
                    updated_at = $7
                WHERE harvest.id = $8;
            "#,
            harvest.cultivar_id.0,
            harvest.location_id.0,
            harvest.price,
            harvest.r#type,
            harvest.description,
            harvest.harvest_date,
            harvest.updated_at,
            id.0,
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Harvest updated successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_harvest_database_error(&err)?;

                tracing::error!("Database error, failed to update harvest: {}", err);
                Err(err.into())
            }
        }
    }

    /// Deletes harvest from the database
    ///
    /// Harvest will only be deleted if it has not stayed on
    /// the platform for at least `HARVEST_MAX_AGE_TO_ARCHIVE` days
    #[tracing::instrument(name = "Delete Harvest", skip(db))]
    pub async fn delete(id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        let mut harvest = find_delete_harvest(id, db.clone()).await?;
        let images = harvest.images.take();

        delete_or_archive_harvest(harvest, db).await?;

        // Delete harvest images
        if let Some(image_paths) = images {
            let paths = image_paths.clone();
            if delete_harvest_photos(paths.into_iter()).await.is_err() {
                tracing::error!("Io error, failed to delete harvest images at: {image_paths:?}, but harvest was deleted successfully.");
            }
        }

        Ok(())
    }

    /// Inserts harvest image-paths into the database
    /// Returns paths of old images
    #[tracing::instrument(name = "Database::harvest-insert-image", skip(db))]
    pub async fn insert_photos(
        id: ModelID,
        paths: Vec<String>,
        db: DatabaseConnection,
    ) -> ServerResult<Option<Vec<String>>> {
        match sqlx::query!(
            r#"
                UPDATE services.harvests harvest
                SET images = $1
                WHERE harvest.id = $2

                RETURNING (
                    SELECT harvest.images
                    FROM services.harvests harvest
                    WHERE  harvest.id = $2
                ) AS old_images
           "#,
            &paths[..],
            id.0
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(rec) => {
                tracing::debug!("Harvest image-paths inserted successfully");
                Ok(rec.old_images)
            }
            Err(err) => {
                // Handle database constraint error
                handle_harvest_database_error(&err)?;

                tracing::error!(
                    "Database error, failed to insert harvest image-paths: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Deletes harvest image-paths from the database
    #[tracing::instrument(name = "Database::harvest-delete-image", skip(db))]
    pub async fn delete_photos(id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE services.harvests harvest
                SET images = NULL
                WHERE harvest.id = $1

                RETURNING (
                    SELECT harvest.images
                    FROM services.harvests harvest
                    WHERE  harvest.id = $1
                ) AS images
           "#,
            id.0
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(rec) => {
                tracing::debug!("Harvest image-paths deleted successfully");

                // Delete images from the file system
                if let Some(images) = rec.images {
                    tokio::spawn(async move { delete_harvest_photos(images.into_iter()).await });
                }

                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_harvest_database_error(&err)?;

                tracing::error!("Database error, failed to delete image-paths: {}", err);
                Err(err.into())
            }
        }
    }
}

/// Get boost amount
#[must_use]
fn calc_boost_amount(
    boost_amount: Option<rust_decimal::Decimal>,
    expires_at: Option<time::Date>,
    today: time::Date,
) -> rust_decimal::Decimal {
    if expires_at >= Some(today) {
        boost_amount.unwrap_or_else(|| 0.into())
    } else {
        0.into()
    }
}

/// Handle harvest database constraints errors
#[allow(clippy::cognitive_complexity)]
pub fn handle_harvest_database_error(err: &sqlx::Error) -> ServerResult<()> {
    if let sqlx::Error::Database(db_err) = err {
        // Handle db unique constraints
        if db_err.is_unique_violation() {
            tracing::error!(
                "Database error, harvest with the same cultivar name already exists. {:?}",
                err
            );
            return Err(ServerError::rejection(EndpointRejection::Conflict(
                "Harvest with the same cultivar name already exists.".into(),
            )));
        }
        // Handle db foreign key constraints
        if db_err.is_foreign_key_violation() {
            if let Some(constraint) = db_err.constraint() {
                if constraint == "harvests_cultivar_id_fkey" {
                    tracing::error!("Database error, cultivar not found. {:?}", err);
                    return Err(ServerError::rejection(EndpointRejection::BadRequest(
                        "Cultivar not found.".into(),
                    )));
                }

                if constraint == "harvests_location_id_fkey" {
                    tracing::error!("Database error, location not found. {:?}", err);
                    return Err(ServerError::rejection(EndpointRejection::BadRequest(
                        "Location not found.".into(),
                    )));
                }
            }
            tracing::error!("Database error, location or cultivar not found. {:?}", err);
            return Err(ServerError::rejection(EndpointRejection::BadRequest(
                "Location or cultivar not found.".into(),
            )));
        }
    }

    // For updates only
    if matches!(err, &sqlx::Error::RowNotFound) {
        tracing::error!("Database error, harvest not found. {:?}", err);
        return Err(ServerError::rejection(EndpointRejection::NotFound(
            "Harvest not found.".into(),
        )));
    }

    Ok(())
}
