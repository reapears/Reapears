//! Cultivar forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, FromRequestParts, Json},
    http::Request,
};
use serde::Deserialize;
use tokio::task::JoinSet;
use validator::Validate;

use crate::{
    endpoint::{validators::join_validation_tasks, EndpointRejection},
    server::state::ServerState,
    types::ModelID,
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
    pub id: ModelID,
    pub name: String,
    pub category_id: ModelID,
}

impl From<CultivarCreateForm> for CultivarInsertData {
    fn from(form: CultivarCreateForm) -> Self {
        Self {
            id: ModelID::new(),
            name: form.name,
            category_id: ModelID::from_str_unchecked(&form.category_id),
        }
    }
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for CultivarCreateForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        let Json(input) = Json::<Self>::from_request(req, state).await?;

        match input.validate() {
            Ok(()) => {
                let mut tasks = JoinSet::new();
                let db = state.database.clone();

                // Validate category id
                let category_id_handler = tasks.spawn({
                    let Ok(category_id) = ModelID::try_from(input.category_id.as_str()) else{
                            return Err(EndpointRejection::BadRequest("Category not found".into()));
                    };
                    let db = db.clone();
                    async move { validate_category_id(category_id, db).await }
                });

                // Validate cultivar name
                let name_handler = tasks.spawn({
                    let name = input.name.clone();
                    async move { validate_cultivar_name(name, db).await }
                });

                let task_handlers = [name_handler, category_id_handler];

                // Wait for tasks to finish
                join_validation_tasks(tasks, &task_handlers).await?;

                Ok(input)
            }
            Err(err) => {
                tracing::error!("Validation error: {}", err);
                Err(EndpointRejection::BadRequest(err.to_string().into()))
            }
        }
    }
}

// ===== Cultivar UpdateForm impls ======

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
    pub category_id: Option<ModelID>,
}

impl From<CultivarUpdateForm> for CultivarUpdateData {
    fn from(form: CultivarUpdateForm) -> Self {
        Self {
            category_id: form.category_id.map(ModelID::from_str_unchecked),
            name: form.name,
        }
    }
}

#[async_trait]
impl<B> FromRequest<ServerState, B> for CultivarUpdateForm
where
    Json<Self>: FromRequest<ServerState, B, Rejection = JsonRejection>,
    B: Send + 'static,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request<B>, state: &ServerState) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();
        let cultivar_id = { ModelID::from_request_parts(&mut parts, state).await? };
        let Json(input) =
            Json::<Self>::from_request(Request::from_parts(parts, body), state).await?;

        match input.validate() {
            Ok(()) => {
                let mut tasks = JoinSet::new();
                let mut task_handlers = Vec::new();
                let db = state.database.clone();

                // Validate cultivar id
                let cultivar_id_handler = tasks.spawn({
                    let db = db.clone();
                    async move { validate_cultivar_id(cultivar_id, db).await }
                });
                task_handlers.push(cultivar_id_handler);

                // Validate category id exists
                if let Some(ref category_id) = input.category_id {
                    let category_id_handler = tasks.spawn({
                        let Ok(category_id) = ModelID::try_from(category_id.as_str()) else{
                            return Err(EndpointRejection::BadRequest("Category not found".into()));
                        };
                        let db = db.clone();
                        async move { validate_category_id(category_id, db).await }
                    });
                    task_handlers.push(category_id_handler);
                }

                // Validate cultivar name
                if let Some(name) = input.name.clone() {
                    let name_handler =
                        tasks.spawn(async move { validate_cultivar_name(name, db).await });
                    task_handlers.push(name_handler);
                }

                // Wait for tasks to finish
                join_validation_tasks(tasks, &task_handlers).await?;

                Ok(input)
            }
            Err(err) => {
                tracing::error!("Validation error: {}", err);
                Err(EndpointRejection::BadRequest(err.to_string().into()))
            }
        }
    }
}

// ===== Helpers =====

mod helpers {

    use crate::{
        endpoint::{EndpointRejection, EndpointResult},
        server::state::DatabaseConnection,
        types::ModelID,
    };

    /// Validate cultivar exists.
    pub async fn validate_cultivar_id(id: ModelID, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.cultivars
                    WHERE cultivars.id = $1
                ) AS "exists!"
            "#,
            id.0
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
    pub async fn validate_category_id(id: ModelID, db: DatabaseConnection) -> EndpointResult<()> {
        match sqlx::query!(
            r#"
                SELECT EXISTS(
                    SELECT 1 FROM services.cultivar_categories category
                    WHERE category.id = $1
                ) AS "exists!"
            "#,
            id.0
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
