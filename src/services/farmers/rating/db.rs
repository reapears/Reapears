//! `FarmRating` database impl

use uuid::Uuid;

use crate::{error::ServerResult, server::state::DatabaseConnection, types::Pagination};

use super::{
    forms::{FarmRatingInsertData, FarmRatingUpdateData},
    models::{FarmRating, FarmRatingList},
};

impl FarmRating {
    /// Fetches all farm-rating records from the database
    #[tracing::instrument(name = "Fetch FarmRatingList", skip(db))]
    pub async fn records(pg: Pagination, db: DatabaseConnection) -> ServerResult<FarmRatingList> {
        let (offset, limit) = pg.offset_limit();
        match sqlx::query!(
            r#"
                SELECT farm_rating.id AS farm_rating_id,
                    farm_rating.grade AS farm_rating_grade,
                    farm_rating.comment AS farm_rating_comment,
                    farm_rating.updated_at AS "farm_rating_updated_at?",
                    farm_rating.created_at AS farm_rating_created_at,
                    farm.id AS "farm_id!",
                    farm.name AS "farm_name!",
                    user_.id AS user_id,
                    user_.first_name AS user_first_name,
                    user_.last_name AS user_last_name,
                    user_profile.photo AS user_photo
                FROM services.farm_ratings farm_rating
                LEFT JOIN services.active_farms farm
                    ON farm_rating.farm_id = farm.id
                LEFT JOIN accounts.users user_
                    ON farm_rating.author_id = user_.id
                LEFT JOIN accounts.user_profiles user_profile
                    On farm_rating.author_id = user_profile.user_id

                ORDER BY farm_rating.created_at
                LIMIT $1
                OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let farm_ratings: Vec<_> = records
                    .into_iter()
                    .map(|rec| {
                        Self::from_row(
                            rec.farm_rating_id,
                            rec.farm_rating_grade,
                            rec.farm_rating_comment,
                            rec.farm_rating_updated_at
                                .unwrap_or(rec.farm_rating_created_at),
                            rec.farm_id,
                            rec.farm_name,
                            rec.user_id,
                            rec.user_first_name,
                            rec.user_last_name,
                            rec.user_photo,
                        )
                    })
                    .collect();

                Ok(farm_ratings)
            }
            Err(err) => {
                tracing::error!("Database error, failed to fetch farm-ratings: {}", err);
                Err(err.into())
            }
        }
    }

    /// Fetches farm-rating detail from the database
    #[tracing::instrument(name = "Find FarmRating", skip(db))]
    pub async fn find(id: Uuid, db: DatabaseConnection) -> ServerResult<Option<Self>> {
        match sqlx::query!(
            r#"
                SELECT farm_rating.id AS farm_rating_id,
                    farm_rating.grade AS farm_rating_grade,
                    farm_rating.comment AS farm_rating_comment,
                    farm_rating.updated_at AS "farm_rating_updated_at?",
                    farm_rating.created_at AS farm_rating_created_at,
                    farm.id AS "farm_id!",
                    farm.name AS "farm_name!",
                    user_.id AS user_id,
                    user_.first_name AS user_first_name,
                    user_.last_name AS user_last_name,
                    user_profile.photo AS user_photo
                FROM services.farm_ratings farm_rating
                LEFT JOIN services.active_farms farm
                    ON farm_rating.farm_id = farm.id
                LEFT JOIN accounts.users user_
                    ON farm_rating.author_id = user_.id
                LEFT JOIN accounts.user_profiles user_profile
                    On farm_rating.author_id = user_profile.user_id

                WHERE farm_rating.id = $1;
            "#,
            id,
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(rec) => {
                let farm_rating = Self::from_row(
                    rec.farm_rating_id,
                    rec.farm_rating_grade,
                    rec.farm_rating_comment,
                    rec.farm_rating_updated_at
                        .unwrap_or(rec.farm_rating_created_at),
                    rec.farm_id,
                    rec.farm_name,
                    rec.user_id,
                    rec.user_first_name,
                    rec.user_last_name,
                    rec.user_photo,
                );

                Ok(Some(farm_rating))
            }
            Err(err) => {
                if matches!(err, sqlx::Error::RowNotFound) {
                    Ok(None)
                } else {
                    tracing::error!("Database error, failed to fetch farm-rating: {}", err);
                    Err(err.into())
                }
            }
        }
    }

    /// Inserts farm-rating into the database
    #[tracing::instrument(name = "Insert FarmRating", skip(db, farm_rating))]
    pub async fn insert(
        farm_rating: FarmRatingInsertData,
        db: DatabaseConnection,
    ) -> ServerResult<Uuid> {
        match sqlx::query!(
            r#"
                INSERT INTO services.farm_ratings(
                    id, 
                    author_id, 
                    farm_id, 
                    grade, 
                    comment, 
                    created_at
                )
                VALUES($1, $2, $3, $4, $5, $6)
            "#,
            farm_rating.id,
            farm_rating.user_id,
            farm_rating.farm_id,
            i32::from(farm_rating.grade),
            farm_rating.comment,
            farm_rating.created_at
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Farm-rating inserted successfully: {:?}", result);
                Ok(farm_rating.id)
            }
            Err(err) => {
                tracing::error!("Database error, failed to insert farm-rating: {}", err);
                Err(err.into())
            }
        }
    }

    /// Updates farm-rating in the database
    #[tracing::instrument(name = "Update FarmRating", skip(db, farm_rating))]
    pub async fn update(
        id: Uuid,
        farm_rating: FarmRatingUpdateData,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE services.farm_ratings farm_rating
                SET grade = COALESCE($1, farm_rating.grade),
                    comment = $2,
                    updated_at = $3
                    WHERE farm_rating.id = $4
            "#,
            farm_rating.grade.map(i32::from),
            farm_rating.comment,
            farm_rating.updated_at,
            id,
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Farm-rating updated successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!("Database error, failed to update farm-rating: {}", err);
                Err(err.into())
            }
        }
    }

    /// Deletes farm-rating from the database
    #[tracing::instrument(name = "Delete FarmRating", skip(db))]
    pub async fn delete(id: Uuid, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                DELETE FROM services.farm_ratings farm_rating
                    WHERE farm_rating.id = $1
            "#,
            id
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Farm-rating deleted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!("Database error, failed to delete farm-rating: {}", err);
                Err(err.into())
            }
        }
    }

    // Fetch farm's ratings from the database
    #[allow(dead_code)]
    pub async fn records_for_farm(
        farm_id: Uuid,
        pg: Pagination,
        db: DatabaseConnection,
    ) -> ServerResult<FarmRatingList> {
        let (offset, limit) = pg.offset_limit();
        match sqlx::query!(
            r#"
                SELECT farm_rating.id AS farm_rating_id,
                    farm_rating.grade AS farm_rating_grade,
                    farm_rating.comment AS farm_rating_comment,
                    farm_rating.updated_at AS "farm_rating_updated_at?",
                    farm_rating.created_at AS farm_rating_created_at,
                    farm.id AS "farm_id!",
                    farm.name AS "farm_name!",
                    user_.id AS user_id,
                    user_.first_name AS user_first_name,
                    user_.last_name AS user_last_name,
                    user_profile.photo AS user_photo
                FROM services.farm_ratings farm_rating
                LEFT JOIN services.active_farms farm
                    ON farm_rating.farm_id = farm.id
                LEFT JOIN accounts.users user_
                    ON farm_rating.author_id = user_.id
                LEFT JOIN accounts.user_profiles user_profile
                    On farm_rating.author_id = user_profile.user_id
                
                WHERE farm.id = $1
                ORDER BY farm_rating.created_at
                LIMIT $2
                OFFSET $3
            "#,
            farm_id,
            limit,
            offset
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let farm_ratings: Vec<_> = records
                    .into_iter()
                    .map(|rec| {
                        Self::from_row(
                            rec.farm_rating_id,
                            rec.farm_rating_grade,
                            rec.farm_rating_comment,
                            rec.farm_rating_updated_at
                                .unwrap_or(rec.farm_rating_created_at),
                            rec.farm_id,
                            rec.farm_name,
                            rec.user_id,
                            rec.user_first_name,
                            rec.user_last_name,
                            rec.user_photo,
                        )
                    })
                    .collect();

                Ok(farm_ratings)
            }
            Err(err) => {
                tracing::error!("Database error, failed to fetch farm's ratings: {}", err);
                Err(err.into())
            }
        }
    }
}