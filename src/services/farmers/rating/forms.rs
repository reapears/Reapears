//! Farm rating forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, FromRequestParts, Json, Request},
};
use serde::Deserialize;
use time::OffsetDateTime;

use crate::{
    auth::CurrentUser,
    endpoint::{
        validators::{TransformString, ValidateString},
        EndpointRejection, EndpointResult,
    },
    server::state::ServerState,
    types::ModelID,
};

use super::permissions::check_user_owns_rating;

use helpers::validate_rating_grade;

/// Farm rating create form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FarmRatingCreateForm {
    pub grade: u8,
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
    /// Validates farm rating form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        validate_rating_grade(self.grade)?;

        self.comment
            .validate_len(0, 32, "Comment must be at most 512 characters")?;

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.comment = self.comment.clean();
    }

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
impl FromRequest<ServerState> for FarmRatingCreateForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut rating) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        rating.validate()?;

        Ok(rating)
    }
}

// ===== FarmRating Update form impls ======

/// Farm rating update form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FarmRatingUpdateForm {
    pub grade: u8,
    pub comment: String,
}

/// Farm rating update form
#[derive(Debug, Clone)]
pub struct FarmRatingUpdateData {
    pub grade: u8,
    pub comment: String,
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
    /// Validates farm rating form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        validate_rating_grade(self.grade)?;

        self.comment
            .validate_len(0, 32, "Comment must be at most 512 characters")?;

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.comment = self.comment.clean();
    }

    ///  Validate a user has the permissions to update the rating
    async fn authorize_request(
        user: CurrentUser,
        rating_id: ModelID,
        state: &ServerState,
    ) -> EndpointResult<()> {
        check_user_owns_rating(user.id, rating_id, state.database()).await
    }
}

#[async_trait]
impl FromRequest<ServerState> for FarmRatingUpdateForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let (mut parts, body) = req.into_parts();
        let user = { CurrentUser::from_parts(&mut parts, state).await? };
        let rating_id = { ModelID::from_request_parts(&mut parts, state).await? };
        let Json(mut rating) =
            Json::<Self>::from_request(Request::from_parts(parts, body), state).await?;

        // Validate form fields
        rating.validate()?;

        // Authorize request
        Self::authorize_request(user, rating_id, state).await?;

        Ok(rating)
    }
}

// ===== Helpers =====

mod helpers {
    use crate::endpoint::{EndpointRejection, EndpointResult};

    /// Validate rating `grade` is between 1 and 5.
    pub fn validate_rating_grade(grade: u8) -> EndpointResult<()> {
        if !(1..=5).contains(&grade) {
            return Err(EndpointRejection::BadRequest(
                "Rating grade must be between 1 and 5".into(),
            ));
        }
        Ok(())
    }
}
