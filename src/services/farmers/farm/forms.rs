//! Farm forms impls

use axum::async_trait;
use serde::Deserialize;
use time::{Date, OffsetDateTime};
use uuid::Uuid;
use validator::Validate;

use crate::{
    db,
    endpoint::{EndpointRejection, EndpointResult, ModelId, ValidateForm},
    server::state::ServerState,
    services::farmers::location::forms::{LocationEmbeddedForm, LocationInsertData},
};

pub use helpers::validate_farm_id;

/// Farm create form
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct FarmCreateForm {
    #[validate(length(min = 1, max = 128))]
    pub name: String,

    pub location: LocationEmbeddedForm,
}

/// Farm create cleaned data
#[derive(Debug, Clone)]
pub struct FarmInsertData {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub location: LocationInsertData,
    pub registered_on: Date,
}

impl FarmCreateForm {
    /// Convert `Self` into `FarmInsertData`
    #[allow(dead_code)]
    #[must_use]
    pub fn data(self, user_id: Uuid) -> FarmInsertData {
        let id = db::model_id();
        FarmInsertData {
            id,
            owner_id: user_id,
            name: self.name,
            location: self.location.data(id),
            registered_on: OffsetDateTime::now_utc().date(),
        }
    }
}

#[async_trait]
impl ValidateForm<ServerState> for FarmCreateForm {
    #[tracing::instrument(skip(self, state), name = "Validate FarmCreateForm")]
    async fn validate_form(
        self,
        state: &ServerState,
        _model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        match self.validate() {
            Ok(()) => {
                let location = self.location.clone();
                location.validate_form(state, None).await?;
                Ok(self)
            }
            Err(err) => Err(EndpointRejection::BadRequest(err.to_string().into())),
        }
    }
}

// Update form impls

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

#[async_trait]
impl ValidateForm<ServerState> for FarmUpdateForm {
    #[tracing::instrument(skip(self, state), name = "Validate FarmUpdateForm")]
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
                // Validate farm id exists
                let db = state.database.clone();
                validate_farm_id(farm_id, db).await?;
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

    /// Validate `farm_id` exists
    pub async fn validate_farm_id(id: Uuid, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.active_farms farm
                    WHERE farm.id = $1
                ) AS "exists!"
            "#,
            id
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
