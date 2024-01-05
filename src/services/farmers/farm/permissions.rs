//! Farm permission impls

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::{
    auth::FarmerUser,
    endpoint::{EndpointRejection, EndpointResult},
    server::state::{DatabaseConnection, ServerState},
    types::ModelID,
};

/// Checks if user owns the farm
#[derive(Debug, Clone)]
pub struct FarmOwnershipPermission;

#[async_trait]
impl FromRequestParts<ServerState> for FarmOwnershipPermission {
    type Rejection = EndpointRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServerState,
    ) -> Result<Self, Self::Rejection> {
        let user = FarmerUser::from_parts(parts, state).await?;
        let farm_id = ModelID::from_request_parts(parts, state).await?;

        check_user_owns_farm(user.id(), farm_id, state.database()).await?;

        Ok(Self)
    }
}

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
