//! Farm rating forms impls

use axum::async_trait;
use serde::Deserialize;
use time::OffsetDateTime;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db,
    endpoint::{EndpointRejection, EndpointResult, ModelId, ValidateForm},
    server::state::ServerState,
    services::farmers::farm::forms::validate_farm_id,
};

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
    pub id: Uuid,
    pub farm_id: Uuid,
    pub user_id: Uuid,
    pub grade: u8,
    pub comment: String,
    pub created_at: OffsetDateTime,
}

impl FarmRatingCreateForm {
    /// Convert `Self` into `FarmRatingInsertData`
    #[allow(dead_code)]
    #[must_use]
    pub fn data(self, farm_id: Uuid, user_id: Uuid) -> FarmRatingInsertData {
        FarmRatingInsertData {
            id: db::model_id(),
            farm_id,
            user_id,
            grade: self.grade,
            comment: self.comment,
            created_at: OffsetDateTime::now_utc(),
        }
    }
}

#[async_trait]
impl ValidateForm<ServerState> for FarmRatingCreateForm {
    #[tracing::instrument(skip(self, state), name = "Validate FarmRatingCreateForm")]
    async fn validate_form(
        self,
        state: &ServerState,
        model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        match self.validate() {
            Ok(()) => {
                // extract farm id
                let Some(ModelId(farm_id)) = model_id else {
                    tracing::error!("Could not extract farm_id from request");
                    return Err(EndpointRejection::BadRequest("Farm id not found".into()));
                };
                let db = state.database.clone();
                validate_farm_id(farm_id, db).await?;
                Ok(self)
            }
            Err(err) => Err(EndpointRejection::BadRequest(err.to_string().into())),
        }
    }
}

// Update form impls

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

#[async_trait]
impl ValidateForm<ServerState> for FarmRatingUpdateForm {
    #[tracing::instrument(skip(self, state), name = "Validate FarmRatingUpdateForm")]
    async fn validate_form(
        self,
        state: &ServerState,
        model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        match self.validate() {
            Ok(()) => {
                // extract rating id
                let Some(ModelId(rating_id)) = model_id else {
                    tracing::error!("Could not extract farm rating is from request");
                    return Err(EndpointRejection::BadRequest("Farm rating not found".into()));
                };
                let db = state.database.clone();
                validate_farm_rating_id(rating_id, db).await?;
                Ok(self)
            }
            Err(err) => Err(EndpointRejection::BadRequest(err.to_string().into())),
        }
    }
}

mod helpers {
    use crate::{
        core::server::state::DatabaseConnection,
        endpoint::{EndpointRejection, EndpointResult},
    };
    use uuid::Uuid;

    /// Validate `rating_id` exists
    pub async fn validate_farm_rating_id(id: Uuid, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.farm_ratings rating
                    WHERE rating.id = $1
                ) AS "exists!"
            "#,
            id
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
