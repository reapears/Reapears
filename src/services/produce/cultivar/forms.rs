//! Cultivar forms impls

use axum::async_trait;
use serde::Deserialize;
use tokio::task::JoinSet;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db,
    endpoint::{
        validators::{join_validation_tasks, parse_uuid, unwrap_uuid},
        EndpointRejection, EndpointResult, ModelId, ValidateForm,
    },
    server::state::ServerState,
};

pub use helpers::validate_cultivar_id;
use helpers::{validate_category_id, validate_cultivar_name};

/// Cultivar create form
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CultivarCreateForm {
    #[validate(length(min = 1, max = 32))]
    pub name: String,

    #[validate(length(min = 1, max = 64, message = "Invalid category id"))]
    pub category_id: String,
}

/// Cultivar create form cleaned data
#[derive(Debug, Clone)]
pub struct CultivarInsertData {
    pub id: Uuid,
    pub name: String,
    pub category_id: Uuid,
}

impl From<CultivarCreateForm> for CultivarInsertData {
    fn from(form: CultivarCreateForm) -> Self {
        Self {
            id: db::model_id(),
            name: form.name,
            // Safety: the id is validated already by create form
            category_id: unwrap_uuid(&form.category_id),
        }
    }
}

#[async_trait]
impl ValidateForm<ServerState> for CultivarCreateForm {
    #[tracing::instrument(skip(self, state), name = "Validate CultivarCreateForm")]
    async fn validate_form(
        self,
        state: &ServerState,
        _model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        match self.validate() {
            Ok(()) => {
                // validate category_id is valid Uuid
                let category_id = parse_uuid(
                    &self.category_id,
                    "Category not found",
                    "Invalid category id",
                )?;

                let mut tasks = JoinSet::new();
                let db = state.database.clone();
                let name_conn = db.clone();
                let name = self.name.clone();
                let name_handler =
                    tasks.spawn(async move { validate_cultivar_name(name, name_conn).await });
                let category_id_handler =
                    tasks.spawn(async move { validate_category_id(category_id, db).await });

                let task_handlers = [name_handler, category_id_handler];

                // Wait for tasks to finish
                join_validation_tasks(tasks, &task_handlers).await?;

                Ok(self)
            }
            Err(err) => {
                tracing::error!("Validation error: {}", err);
                Err(EndpointRejection::BadRequest(err.to_string().into()))
            }
        }
    }
}

// UpdateForm impls

/// Cultivar update form
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CultivarUpdateForm {
    #[validate(length(min = 1, max = 32))]
    pub name: Option<String>,

    #[validate(length(min = 1, max = 64, message = "Invalid category id"))]
    pub category_id: Option<String>,
}

/// Cultivar update form cleaned data
#[derive(Debug, Clone)]
pub struct CultivarUpdateData {
    pub name: Option<String>,
    pub category_id: Option<Uuid>,
}

impl From<CultivarUpdateForm> for CultivarUpdateData {
    fn from(form: CultivarUpdateForm) -> Self {
        Self {
            category_id: form.category_id.map(|id| unwrap_uuid(&id)),
            name: form.name,
        }
    }
}

#[async_trait]
impl ValidateForm<ServerState> for CultivarUpdateForm {
    #[tracing::instrument(skip(self, state), name = "Validate CultivarUpdateForm")]
    async fn validate_form(
        self,
        state: &ServerState,
        model_id: Option<ModelId<Uuid>>,
    ) -> EndpointResult<Self> {
        match self.validate() {
            Ok(()) => {
                let mut tasks = JoinSet::new();
                let mut task_handlers = Vec::new();
                let db = state.database.clone();
                let cultivar_id_conn = db.clone();

                // extract cultivar id
                let Some(ModelId(cultivar_id)) = model_id else {
                    tracing::error!("Could not extract cultivar_id from request");
                    return Err(EndpointRejection::BadRequest("Cultivar id not found".into()));
                };
                let cultivar_id_handler = tasks.spawn(async move {
                    validate_cultivar_id(cultivar_id, cultivar_id_conn).await
                });
                task_handlers.push(cultivar_id_handler);

                // validate category id exists
                if let Some(category_id) = self.category_id.clone() {
                    let db = db.clone();
                    let category_id = parse_uuid(
                        &category_id,
                        "Cultivar category not found",
                        "Invalid category id",
                    )?;
                    let category_id_handler =
                        tasks.spawn(async move { validate_category_id(category_id, db).await });
                    task_handlers.push(category_id_handler);
                }

                // validate cultivar name
                if let Some(name) = self.name.clone() {
                    let name_handler =
                        tasks.spawn(async move { validate_cultivar_name(name, db).await });
                    task_handlers.push(name_handler);
                }

                // Wait for tasks to finish
                join_validation_tasks(tasks, &task_handlers).await?;

                Ok(self)
            }
            Err(err) => {
                tracing::error!("Validation error: {}", err);
                Err(EndpointRejection::BadRequest(err.to_string().into()))
            }
        }
    }
}

mod helpers {

    use uuid::Uuid;

    use crate::{
        endpoint::{EndpointRejection, EndpointResult},
        server::state::DatabaseConnection,
    };

    /// Validate cultivar exists.
    pub async fn validate_cultivar_id(id: Uuid, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.cultivars
                    WHERE cultivars.id = $1
                ) AS "exists!"
            "#,
            id
        )
        .fetch_one(&db.pool)
        .await
        {
            // Returns ok is the name does not exists
            Ok(row) => {
                if row.exists {
                    Ok(())
                } else {
                    tracing::error!("Cultivar id not found: {id}");
                    Err(EndpointRejection::BadRequest("Cultivar not found.".into()))
                }
            }
            Err(err) => {
                tracing::error!("Database error: {}", err);
                Err(EndpointRejection::internal_server_error())
            }
        }
    }

    /// Validate cultivar name does not exists already.
    pub async fn validate_cultivar_name(
        name: String,
        db: DatabaseConnection,
    ) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.cultivars
                    WHERE LOWER(cultivars.name) = LOWER($1)
                ) AS "exists!"
            "#,
            name.clone()
        )
        .fetch_one(&db.pool)
        .await
        {
            // Returns ok is the name does not exists
            Ok(row) => {
                if row.exists {
                    tracing::error!("Cultivar: '{}' already exists.", name);
                    Err(EndpointRejection::BadRequest(
                        "Cultivar already exists.".into(),
                    ))
                } else {
                    Ok(())
                }
            }
            Err(err) => {
                tracing::error!("Database error: {}", err);
                Err(EndpointRejection::internal_server_error())
            }
        }
    }

    /// Validate `category_id` exists
    ///
    /// # Errors
    ///
    /// Return an error if category id cannot be found
    pub async fn validate_category_id(id: Uuid, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.cultivar_categories category
                    WHERE category.id = $1
                ) AS "exists!"
            "#,
            id
        )
        .fetch_one(&db.pool)
        .await
        {
            // Returns ok is the category_id exists
            Ok(row) => {
                if row.exists {
                    Ok(())
                } else {
                    tracing::error!("Cultivar category_id: '{}' does not exists.", id);
                    Err(EndpointRejection::BadRequest("Category not found.".into()))
                }
            }
            Err(err) => {
                tracing::error!("Database error: {}", err);
                Err(EndpointRejection::internal_server_error())
            }
        }
    }
}
