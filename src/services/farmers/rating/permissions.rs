//! `FarmRating` permission impls

use crate::{
    endpoint::{EndpointRejection, EndpointResult},
    server::state::DatabaseConnection,
    types::ModelID,
};

/// Validate user owns the rating
pub async fn check_user_owns_rating(
    user_id: ModelID,
    rating_id: ModelID,
    db: DatabaseConnection,
) -> EndpointResult<()> {
    match sqlx::query!(
        r#"
            SELECT first_name
            FROM accounts.users user_
            LEFT JOIN services.farm_ratings farm_rating
                ON user_.id = farm_rating.author_id
            WHERE (
                user_.id = $1
                AND farm_rating.id = $2
            )
            "#,
        user_id.0,
        rating_id.0
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
