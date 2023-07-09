//! Farm forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, FromRequestParts, Json},
    http::Request,
};
use serde::Deserialize;
use time::{Date, OffsetDateTime};
use validator::Validate;

use crate::{
    auth::FarmerUser,
    endpoint::{EndpointRejection, EndpointResult},
    server::state::ServerState,
    services::farmers::location::forms::{LocationEmbeddedForm, LocationInsertData},
    types::ModelID,
};

use super::permissions::check_user_owns_farm;

pub use helpers::validate_farm_id;

/// Farm create form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct FarmCreateForm {
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    pub contact_email: Option<String>,
    pub contact_number: Option<String>,
    pub location: LocationEmbeddedForm,
}

/// Farm create cleaned data
#[derive(Debug, Clone)]
pub struct FarmInsertData {
    pub id: ModelID,
    pub owner_id: ModelID,
    pub name: String,
    pub contact_email: Option<String>,
    pub contact_number: Option<String>,
    pub location: LocationInsertData,
    pub registered_on: Date,
}

impl FarmCreateForm {
    /// Convert `Self` into `FarmInsertData`
    #[allow(dead_code)]
    #[must_use]
    pub fn data(self, user_id: ModelID) -> FarmInsertData {
        let id = ModelID::new();
        FarmInsertData {
            id,
            owner_id: user_id,
            name: self.name,
            contact_email: self.contact_email,
            contact_number: self.contact_number,
            location: self.location.data(id),
            registered_on: OffsetDateTime::now_utc().date(),
        }
    }
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for FarmCreateForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        let Json(input) = Json::<Self>::from_request(req, state).await?;

        match input.validate() {
            Ok(()) => {
                input.location.validate_form(state).await?;
                Ok(input)
            }
            Err(err) => Err(EndpointRejection::BadRequest(err.to_string().into())),
        }
    }
}

// ===== Farm Update form impls ======

/// Farm update form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct FarmUpdateForm {
    #[validate(length(min = 1, max = 128))]
    pub name: String,
}

/// Farm update cleaned data
#[derive(Debug, Clone)]
pub struct FarmUpdateData {
    pub name: String,
}

impl From<FarmUpdateForm> for FarmUpdateData {
    fn from(form: FarmUpdateForm) -> Self {
        Self { name: form.name }
    }
}

impl FarmUpdateForm {
    ///  Validate a user has the permissions to update a farm
    async fn authorize_request(
        state: &ServerState,
        user: FarmerUser,
        farm_id: ModelID,
    ) -> EndpointResult<()> {
        check_user_owns_farm(user.id(), farm_id, state.database.clone()).await
    }
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for FarmUpdateForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();
        let user = { FarmerUser::from_parts(&mut parts, state).await? };
        let farm_id = { ModelID::from_request_parts(&mut parts, state).await? };
        let Json(input) =
            Json::<Self>::from_request(Request::from_parts(parts, body), state).await?;

        // Authorize request
        Self::authorize_request(state, user, farm_id).await?;

        match input.validate() {
            Ok(()) => {
                // Validate farm id exists
                let db = state.database.clone();
                validate_farm_id(farm_id, db).await?;
                Ok(input)
            }
            Err(err) => Err(EndpointRejection::BadRequest(err.to_string().into())),
        }
    }
}

mod helpers {
    use crate::types::ModelID;
    use crate::{
        core::server::state::DatabaseConnection,
        endpoint::{EndpointRejection, EndpointResult},
    };

    /// Validate `farm_id` exists
    pub async fn validate_farm_id(id: ModelID, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.active_farms farm
                    WHERE farm.id = $1
                ) AS "exists!"
            "#,
            id.0
        )
        .fetch_one(&db.pool)
        .await
        {
            // Returns ok is the farm id exists
            Ok(row) => {
                if row.exists {
                    Ok(())
                } else {
                    tracing::error!("Farm id: '{}' does not exists.", id);
                    Err(EndpointRejection::BadRequest("Farm not found.".into()))
                }
            }
            Err(err) => {
                tracing::error!("Database error: {}", err);
                Err(EndpointRejection::internal_server_error())
            }
        }
    }
}
