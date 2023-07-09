//! Location permission impls

use crate::{
    endpoint::{EndpointRejection, EndpointResult},
    server::state::DatabaseConnection,
    types::ModelID,
};

/// Validate  user owns a location
pub async fn check_user_owns_location(
    user_id: ModelID,
    location_id: ModelID,
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
            WHERE (
                user_.id = $1
                AND location_.id = $2
            )
        "#,
        user_id.0,
        location_id.0
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
