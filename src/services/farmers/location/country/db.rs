//! Location country database impl

use crate::{
    endpoint::EndpointRejection,
    error::{ServerError, ServerResult},
    server::state::DatabaseConnection,
    types::ModelID,
};

use super::{
    forms::{CountryInsertData, CountryUpdateData},
    Country, CountryList,
};

impl Country {
    /// Fetches Location country records from the database
    #[tracing::instrument(name = "Database::records-location-countries", skip(db))]
    pub async fn records(db: DatabaseConnection) -> ServerResult<CountryList> {
        match sqlx::query!(
            r#"
                SELECT  country.id,
                     country.name
                FROM services.countries country
            "#
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let countries = records
                    .into_iter()
                    .map(|rec| Self::from_row(rec.id.into(), rec.name))
                    .collect();

                Ok(countries)
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to fetch location countries: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Inserts location country into the database
    #[tracing::instrument(name = "Insert Location-country", skip(db, country))]
    pub async fn insert(
        country: CountryInsertData,
        db: DatabaseConnection,
    ) -> ServerResult<ModelID> {
        match sqlx::query!(
            r#"
                INSERT INTO services.countries (
                    id, 
                    name
                )
                VALUES ($1, $2);
            "#,
            country.id.0,
            country.name
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Location country inserted successfully: {:?}", result);
                Ok(country.id)
            }
            Err(err) => {
                // Handle database constraint error
                handle_country_database_error(&err)?;

                tracing::error!("Database error, failed to insert location country: {}", err);
                Err(err.into())
            }
        }
    }

    /// Updates location country in the database
    #[tracing::instrument(name = "Update Location-country", skip(db, country))]
    pub async fn update(
        id: ModelID,
        country: CountryUpdateData,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE services.countries country
                SET name = COALESCE($1, country.name)
                WHERE country.id = $2
           "#,
            country.name,
            id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Location country updated successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_country_database_error(&err)?;

                tracing::error!("Database error, failed to update location country: {}", err);
                Err(err.into())
            }
        }
    }

    /// Deletes location country from the database
    #[tracing::instrument(name = "Delete Location-country", skip(db))]
    pub async fn delete(id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                DELETE FROM services.countries country
                WHERE country.id = $1
           "#,
            id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Location country deleted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_country_database_error(&err)?;

                tracing::error!("Database error, failed to delete location country: {}", err);
                Err(err.into())
            }
        }
    }
}

/// Handle countries database constraints errors
#[allow(clippy::cognitive_complexity)]
fn handle_country_database_error(err: &sqlx::Error) -> ServerResult<()> {
    if let sqlx::Error::Database(db_err) = err {
        // Handle db unique constraints
        if db_err.is_unique_violation() {
            tracing::error!("Database error, Country already exists. {:?}", err);
            return Err(ServerError::rejection(EndpointRejection::Conflict(
                "Country already exists.".into(),
            )));
        }
    }

    // For updates only
    if matches!(err, &sqlx::Error::RowNotFound) {
        tracing::error!("Database error, country not found. {:?}", err);
        return Err(ServerError::rejection(EndpointRejection::NotFound(
            "Country not found.".into(),
        )));
    }

    Ok(())
}
