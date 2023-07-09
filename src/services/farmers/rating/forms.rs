//! Farm rating forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, FromRequestParts, Json},
    http::Request,
};
use serde::Deserialize;
use time::OffsetDateTime;
use validator::Validate;

use crate::{
    auth::CurrentUser,
    endpoint::{EndpointRejection, EndpointResult},
    server::state::ServerState,
    services::farmers::farm::forms::validate_farm_id,
    types::ModelID,
};

use super::permissions::check_user_owns_rating;

use helpers::validate_farm_rating_id;

/// Farm rating create form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct FarmRatingCreateForm {
    #[validate(range(min = 1, max = 5, message = "Rating must be between 1 and 5"))]
    pub grade: u8,
    #[validate(length(min = 1, max = 512))]
    pub comment: String,
}

/// Farm rating cleaned data
#[derive(Debug, Clone)]
pub struct FarmRatingInsertData {
    pub id: ModelID,
    pub farm_id: ModelID,
    pub user_id: ModelID,
    pub grade: u8,
    pub comment: String,
    pub created_at: OffsetDateTime,
}

impl FarmRatingCreateForm {
    /// Convert `Self` into `FarmRatingInsertData`
    #[allow(dead_code)]
    #[must_use]
    pub fn data(self, farm_id: ModelID, user_id: ModelID) -> FarmRatingInsertData {
        FarmRatingInsertData {
            id: ModelID::new(),
            farm_id,
            user_id,
            grade: self.grade,
            comment: self.comment,
            created_at: OffsetDateTime::now_utc(),
        }
    }
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for FarmRatingCreateForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();
        let farm_id = { ModelID::from_request_parts(&mut parts, state).await? };
        let Json(input) =
            Json::<Self>::from_request(Request::from_parts(parts, body), state).await?;

        match input.validate() {
            Ok(()) => {
                let db = state.database.clone();
                validate_farm_id(farm_id, db).await?;
                Ok(input)
            }
            Err(err) => Err(EndpointRejection::BadRequest(err.to_string().into())),
        }
    }
}

// ===== FarmRating Update form impls ======

/// Farm rating update form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct FarmRatingUpdateForm {
    #[validate(range(min = 1, max = 5, message = "Rating must be between 1 and 5"))]
    pub grade: Option<u8>,
    #[validate(length(min = 1, max = 512))]
    pub comment: Option<String>,
}

/// Farm rating update form
#[derive(Debug, Clone)]
pub struct FarmRatingUpdateData {
    pub grade: Option<u8>,
    pub comment: Option<String>,
    pub updated_at: OffsetDateTime,
}

impl From<FarmRatingUpdateForm> for FarmRatingUpdateData {
    fn from(form: FarmRatingUpdateForm) -> Self {
        Self {
            grade: form.grade,
            comment: form.comment,
            updated_at: OffsetDateTime::now_utc(),
        }
    }
}

impl FarmRatingUpdateForm {
    ///  Validate a user has the permissions to update the rating
    async fn authorize_request(
        user: CurrentUser,
        rating_id: ModelID,
        state: &ServerState,
    ) -> EndpointResult<()> {
        check_user_owns_rating(user.id, rating_id, state.database.clone()).await
    }
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for FarmRatingUpdateForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();
        let user = { CurrentUser::from_parts(&mut parts, state).await? };
        let rating_id = { ModelID::from_request_parts(&mut parts, state).await? };
        let Json(input) =
            Json::<Self>::from_request(Request::from_parts(parts, body), state).await?;

        // Authorize request
        Self::authorize_request(user, rating_id, state).await?;

        match input.validate() {
            Ok(()) => {
                let db = state.database.clone();
                validate_farm_rating_id(rating_id, db).await?;
                Ok(input)
            }
            Err(err) => Err(EndpointRejection::BadRequest(err.to_string().into())),
        }
    }
}

// ===== Helpers =====

mod helpers {
    use crate::{
        core::server::state::DatabaseConnection,
        endpoint::{EndpointRejection, EndpointResult},
        types::ModelID,
    };

    /// Validate `rating_id` exists
    pub async fn validate_farm_rating_id(
        id: ModelID,
        db: DatabaseConnection,
    ) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.farm_ratings rating
                    WHERE rating.id = $1
                ) AS "exists!"
            "#,
            id.0
        )
        .fetch_one(&db.pool)
        .await
        {
            Ok(row) => {
                if row.exists {
                    Ok(())
                } else {
                    tracing::error!("Farm rating id: '{}' does not exists.", id);
                    Err(EndpointRejection::BadRequest(
                        "Farm rating not found.".into(),
                    ))
                }
            }
            Err(err) => {
                tracing::error!("Database error: {}", err);
                Err(EndpointRejection::internal_server_error())
            }
        }
    }
}
