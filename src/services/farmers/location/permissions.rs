//! Location permission impls

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::{
    auth::FarmerUser,
    endpoint::{EndpointRejection, EndpointResult},
    server::state::{DatabaseConnection, ServerState},
    types::ModelID,
};

/// Checks if user can delete location
#[derive(Debug, Clone)]
pub struct LocationDeletePermission;

#[async_trait]
impl FromRequestParts<ServerState> for LocationDeletePermission {
    type Rejection = EndpointRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        let user = FarmerUser::from_parts(parts, state).await?;
        let location_id = ModelID::from_request_parts(parts, state).await?;

        check_user_owns_location(user.id(), location_id, state.database()).await?;
        let Some(count) = get_location_count(location_id, state.database()).await? else {
            return Err(EndpointRejection::forbidden());
        };

        if count < 2 {
            return Err(EndpointRejection::Forbidden(
                "Cannot delete farm's only location.".into(),
            ));
        }

        Ok(Self)
    }
}

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

/// Validate  user owns a location
pub async fn get_location_count(
    location_id: ModelID,
    db: DatabaseConnection,
) -> EndpointResult<Option<i64>> {
    match sqlx::query!(
        r#"
            SELECT COUNT(location_.id) AS "location_count"
            FROM services.active_locations location_

            WHERE location_.farm_id IN (
                SELECT location_.farm_id
                FROM services.locations location_
                WHERE location_.id = $1
            )
        "#,
        location_id.0,
    )
    .fetch_one(&db.pool)
    .await
    {
        Ok(rec) => Ok(rec.location_count),
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
