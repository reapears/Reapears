//! Cultivar category database impl

use crate::{error::ServerResult, server::state::DatabaseConnection, types::ModelID};

use super::{
    forms::{CultivarCategoryInsertData, CultivarCategoryUpdateData},
    CategoryList, CultivarCategory,
};

impl CultivarCategory {
    /// Fetches Cultivar category records from the database
    #[tracing::instrument(name = "Database::records-cultivar-category", skip(db))]
    pub async fn records(db: DatabaseConnection) -> ServerResult<CategoryList> {
        match sqlx::query!(
            r#"
                SELECT  category.id,
                     category.name
                FROM services.cultivar_categories category
            "#
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let categories = records
                    .into_iter()
                    .map(|rec| Self::from_row(rec.id.into(), rec.name))
                    .collect();

                Ok(categories)
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to fetch cultivar categories: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Inserts cultivar category into the database
    #[tracing::instrument(name = "Insert Cultivar-category", skip(db, category))]
    pub async fn insert(
        category: CultivarCategoryInsertData,
        db: DatabaseConnection,
    ) -> ServerResult<ModelID> {
        match sqlx::query!(
            r#"
                INSERT INTO services.cultivar_categories (
                    id, 
                    name
                )
                VALUES ($1, $2);
            "#,
            category.id.0,
            category.name
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Cultivar category inserted successfully: {:?}", result);
                Ok(category.id)
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to insert cultivar-category: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Updates cultivar category in the database
    #[tracing::instrument(name = "Update Cultivar-category", skip(db, category))]
    pub async fn update(
        id: ModelID,
        category: CultivarCategoryUpdateData,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE services.cultivar_categories category
                SET name = COALESCE($1, category.name)
                WHERE category.id = $2
           "#,
            category.name,
            id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Cultivar-category updated successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to update cultivar-category: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Deletes cultivar category from the database
    #[tracing::instrument(name = "Delete Cultivar-category", skip(db))]
    pub async fn delete(id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                DELETE FROM services.cultivar_categories category
                WHERE category.id = $1
           "#,
            id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Cultivar-category deleted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to delete cultivar-category: {}",
                    err
                );
                Err(err.into())
            }
        }
    }
}
