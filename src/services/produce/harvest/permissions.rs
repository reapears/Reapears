//! Harvest permission impls

use crate::{
    endpoint::{EndpointRejection, EndpointResult},
    server::state::DatabaseConnection,
    types::ModelID,
};

/// Validate user owns the harvest
///
/// # Errors
///
/// Return an error if user and harvest cannot be found
pub async fn check_user_owns_harvest(
    user_id: ModelID,
    harvest_id: ModelID,
    db: DatabaseConnection,
) -> EndpointResult<()> {
    match sqlx::query!(
        r#"
            SELECT first_name
            FROM accounts.users user_
            LEFT JOIN services.farms farm
                ON user_.id = farm.owner_id
            LEFT JOIN services.locations location_
                ON farm.id = location_.farm_id
            LEFT JOIN services.harvests harvest
            ON location_.id = harvest.location_id
            WHERE (
                user_.id = $1
                AND harvest.id = $2
            )
            "#,
        user_id.0,
        harvest_id.0
    )
    .fetch_one(&db.pool)
    .await
    {
        Ok(_user) => Ok(()),
        Err(err) => {
            if matches!(err, sqlx::Error::RowNotFound) {
                Err(EndpointRejection::forbidden())
            } else {
                tracing::error!("Database error: {}", err);
                Err(EndpointRejection::internal_server_error())
            }
        }
    }
}

/// Validate a user has permissions to update a harvest,
///
/// # Errors
///
/// Return an error if user and harvest cannot be found
pub async fn check_user_can_update_harvest(
    user_id: ModelID,
    location_id: ModelID,
    harvest_id: ModelID,
    db: DatabaseConnection,
) -> EndpointResult<()> {
    match sqlx::query!(
        r#"
            SELECT first_name
            FROM accounts.users user_
            LEFT JOIN services.farms farm
                ON user_.id = farm.owner_id
            LEFT JOIN services.locations location_
                ON farm.id = location_.farm_id
            LEFT JOIN services.harvests harvest
            ON location_.id = harvest.location_id
            WHERE (
                user_.id = $1
                AND location_.id = $2
                AND harvest.id = $3
            )
            "#,
        user_id.0,
        location_id.0,
        harvest_id.0
    )
    .fetch_one(&db.pool)
    .await
    {
        Ok(_user) => Ok(()),
        Err(err) => {
            if matches!(err, sqlx::Error::RowNotFound) {
                Err(EndpointRejection::forbidden())
            } else {
                tracing::error!("Database error: {}", err);
                Err(EndpointRejection::internal_server_error())
            }
        }
    }
}
