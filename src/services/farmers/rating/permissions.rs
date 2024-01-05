//! `FarmRating` permission impls

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::{
    auth::CurrentUser,
    endpoint::{EndpointRejection, EndpointResult},
    server::state::{DatabaseConnection, ServerState},
    types::ModelID,
};

/// Checks if user owns the rating
#[derive(Debug, Clone)]
pub struct FarmRatingOwnershipPermission;

#[async_trait]
impl FromRequestParts<ServerState> for FarmRatingOwnershipPermission {
    type Rejection = EndpointRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        let user = CurrentUser::from_parts(parts, state).await?;
        let rating_id = ModelID::from_request_parts(parts, state).await?;

        check_user_owns_rating(user.id(), rating_id, state.database()).await?;

        Ok(Self)
    }
}

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
