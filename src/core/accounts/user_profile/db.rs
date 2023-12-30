//! User profile database impl

use std::path::PathBuf;

use itertools::Itertools;

use crate::{
    endpoint::EndpointRejection,
    error::{ServerError, ServerResult},
    files,
    server::state::DatabaseConnection,
    services::{
        farmers::{farm::models::Farm, location::models::Location},
        produce::harvest::models::HarvestIndex,
    },
    types::ModelID,
};

use super::{forms::UserProfileUpdateData, models::UserProfile};

// UserProfile
impl UserProfile {
    /// Fetch user-profile from the database
    #[tracing::instrument(skip(db), name = "Find UserProfile")]
    pub async fn find(id: ModelID, db: DatabaseConnection) -> ServerResult<Option<Self>> {
        match sqlx::query!(
            r#"
            SELECT user_.id AS user_id,
                user_.first_name AS user_first_name,
                user_.last_name AS user_last_name,
                user_.date_joined AS user_date_joined,
                profile.about AS "user_about?",
                profile.photo AS user_photo,
                profile.lives_at AS user_lives_at,
                farm.id AS "farm_id?",
                farm.name AS "farm_name?",
                farm.logo AS "farm_logo",
                farm.contact_email AS "farm_contact_email",
                farm.contact_number AS "farm_contact_number",
                farm.registered_on AS "farm_registered_on?",
                location_.id AS "location_id?",
                location_.place_name AS "location_place_name?",
                location_.coords AS location_coords,
                location_.description AS location_description,
                country.name AS "location_country?",
                region.name AS "location_region?",
                harvest.id AS "harvest_id?",
                harvest.price AS "harvest_price?",
                harvest.images AS harvest_images,
                harvest.harvest_date AS "harvest_harvest_date?",
                cultivar.name AS "cultivar_name?",
                cultivar_category.name AS "cultivar_category?",
                cultivar.image AS cultivar_image
            FROM accounts.users user_
            LEFT JOIN accounts.user_profiles profile
                ON user_.id = profile.user_id
            LEFT JOIN services.active_farms farm
                ON user_.id = farm.owner_id
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

            WHERE user_.id = $1
            ORDER BY harvest.created_at
        "#,
            id.0
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) if records.is_empty() => Ok(None),
            Ok(records) => {
                let first_rec = &records[0];

                let user_id = first_rec.user_id.into();
                let first_name = first_rec.user_first_name.clone();
                let last_name = first_rec.user_last_name.clone();
                let about = first_rec.user_about.clone().unwrap_or_default();
                let photo = first_rec.user_photo.clone();
                let lives_at = first_rec.user_lives_at.clone();
                let date_joined = first_rec.user_date_joined.date();

                let mut farms = Vec::new();

                // Create user farms if they have some
                for (farm_id, farm_group) in &records
                    .into_iter()
                    .filter(|rec| rec.farm_id.is_some())
                    .group_by(|rec| rec.farm_id.unwrap())
                {
                    let farm_group: Vec<_> = farm_group.collect();
                    let first_rec = &farm_group[0];

                    let farm_name = first_rec.farm_name.clone().unwrap();
                    let farm_logo = first_rec.farm_logo.clone();
                    let farm_contact_email = first_rec.farm_contact_email.clone();
                    let farm_contact_number = first_rec.farm_contact_number.clone();
                    let registered_on = first_rec.farm_registered_on.unwrap();

                    // Create farm locations
                    let farm_locations = {
                        let mut locations = Vec::new();
                        for (location_id, location_group) in &farm_group
                            .into_iter()
                            .group_by(|rec| rec.location_id.unwrap())
                        {
                            let location_group: Vec<_> = location_group.collect();
                            let first_rec = &location_group[0];

                            let place_name = first_rec.location_place_name.clone().unwrap();
                            let region = first_rec.location_region.clone();
                            let country = first_rec.location_country.clone().unwrap();
                            let coords = first_rec.location_coords.clone();
                            let description = first_rec.location_description.clone();

                            // Create harvests if there is some available at the location.
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

                            locations.push(Location::from_row(
                                location_id.into(),
                                place_name,
                                region,
                                country,
                                coords,
                                description,
                                farm_id.into(),
                                farm_name.clone(),
                                harvests,
                            ));
                        }
                        locations
                    };

                    farms.push(Farm::from_row(
                        farm_id.into(),
                        farm_name,
                        farm_logo,
                        farm_contact_email,
                        farm_contact_number,
                        farm_locations,
                        registered_on,
                        user_id,
                        first_name.clone(),
                        last_name.clone(),
                        photo.clone(),
                    ));
                }

                let farms: Option<Vec<_>> = (!farms.is_empty()).then_some(farms);

                let user_profile = Self::from_row(
                    user_id,
                    first_name,
                    last_name,
                    about,
                    lives_at,
                    photo,
                    date_joined,
                    farms,
                );

                Ok(Some(user_profile))
            }

            Err(err) => {
                tracing::error!("Database error, failed to fetch user-profile: {}", err);
                Err(err.into())
            }
        }
    }

    /// Insert user-profile in the database
    #[tracing::instrument(skip(tx), name = "Insert UserProfile")]
    pub async fn insert_default(
        user_id: ModelID,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> ServerResult<()> {
        let profile = UserProfileUpdateData::default();
        match sqlx::query!(
            r#"
                INSERT INTO accounts.user_profiles(
                    user_id,
                    about, 
                    lives_at
                )
                VALUES($1, $2, $3)
            "#,
            user_id.0,
            profile.about,
            profile.lives_at,
        )
        .execute(&mut **tx)
        .await
        {
            Ok(result) => {
                tracing::debug!(
                    "User-profile inserted successfully, but transaction not committed: {:?}.",
                    result
                );
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_user_profile_database_error(&err)?;

                tracing::error!("Database error, failed to insert user-profile: {}", err);
                Err(err.into())
            }
        }
    }

    /// Insert or Update user-profile in the database
    #[tracing::instrument(skip(db, values), name = "Insert or Update UserProfile")]
    pub async fn create_or_update(
        id: ModelID,
        values: UserProfileUpdateData,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                INSERT INTO accounts.user_profiles(
                    user_id,
                    about, 
                    lives_at
                )
                VALUES($1, $2, $3)

                ON CONFLICT ON CONSTRAINT user_profiles_pkey
                DO UPDATE SET about = EXCLUDED.about,
                            lives_at = EXCLUDED.lives_at;
            "#,
            id.0,
            values.about,
            values.lives_at,
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("User-profile updated successfully: {:?}.", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_user_profile_database_error(&err)?;

                tracing::error!("Database error, failed to update user-profile: {}", err);
                Err(err.into())
            }
        }
    }

    /// Insert user profile photo-path into the database
    #[tracing::instrument(skip(db), name = "Insert profile photo-path")]
    pub async fn insert_photo(
        id: ModelID,
        paths: Vec<PathBuf>,
        db: DatabaseConnection,
    ) -> ServerResult<(String, Option<String>)> {
        let path = files::get_jpg_path(paths)?;
        match sqlx::query!(
            r#"
                UPDATE accounts.user_profiles profile
                SET photo = $1
                WHERE profile.user_id = $2

                RETURNING (
                    SELECT profile.photo
                    FROM accounts.user_profiles profile
                    WHERE profile.user_id = $2
                ) AS old_photo
           "#,
            path,
            id.0
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(rec) => {
                tracing::debug!("User photo-path inserted successfully");
                Ok((path, rec.old_photo))
            }
            Err(err) => {
                // Handle database constraint error
                handle_user_profile_database_error(&err)?;

                tracing::error!("Database error, failed to set user photo-path: {}", err);
                Err(err.into())
            }
        }
    }
}

/// Handle use profile database constraints errors
#[allow(clippy::cognitive_complexity)]
fn handle_user_profile_database_error(err: &sqlx::Error) -> ServerResult<()> {
    // For updates only
    if matches!(err, &sqlx::Error::RowNotFound) {
        tracing::error!("Database error, user not found. {:?}", err);
        return Err(ServerError::rejection(EndpointRejection::NotFound(
            "User not found.".into(),
        )));
    }

    Ok(())
}
