//! Farm permission impls

use crate::{
    endpoint::{EndpointRejection, EndpointResult},
    server::state::DatabaseConnection,
    types::ModelID,
};

/// Validate user owns the farm
pub async fn check_user_owns_farm(
    user_id: ModelID,
    farm_id: ModelID,
    db: DatabaseConnection,
) -> EndpointResult<()> {
    match sqlx::query!(
        r#"
            SELECT first_name
            FROM accounts.users user_
            LEFT JOIN services.farms farm
                ON user_.id = farm.owner_id
            WHERE (
                user_.id = $1
                AND farm.id = $2
            )
            "#,
        user_id.0,
        farm_id.0
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
