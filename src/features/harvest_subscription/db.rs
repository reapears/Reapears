//! Harvest subscription database impl

use crate::{
    endpoint::EndpointRejection,
    error::{ServerError, ServerResult},
    server::state::DatabaseConnection,
    types::ModelID,
};

use super::{
    forms::{HarvestSubscriptionInsertData, HarvestSubscriptionUpdateData},
    models::{HarvestSubscription, HarvestSubscriptionList},
};

impl HarvestSubscription {
    /// Fetches user harvest subscription records from the database
    #[tracing::instrument(name = "Fetch User Harvest Subscriptions", skip(db))]
    pub async fn user_records(
        user_id: ModelID,
        db: DatabaseConnection,
    ) -> ServerResult<HarvestSubscriptionList> {
        match sqlx::query!(
            r#"
                SELECT subscription.id,
                    subscription.harvest_id,
                    subscription.amount,
                    subscription.expires_at,
                    subscription.created_at
                FROM features.harvest_subscriptions subscription
                
                WHERE subscription.harvest_id IN (
                    SELECT harvest.id
                    FROM services.active_farms farm
                    LEFT JOIN services.active_locations location_
                        ON farm.id = location_.farm_id
                    LEFT JOIN services.active_harvests harvest
                        ON location_.id = harvest.location_id
                    
                    WHERE farm.owner_id = $1
                )
            "#,
            user_id.0
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let regions = records
                    .into_iter()
                    .map(|rec| {
                        Self::from_row(
                            rec.id.into(),
                            rec.harvest_id.into(),
                            rec.amount,
                            rec.expires_at,
                            rec.created_at,
                        )
                    })
                    .collect();

                Ok(regions)
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to fetch user harvest subscription: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Fetches harvest subscription records from the database
    #[tracing::instrument(name = "Fetch Harvest Subscriptions", skip(db))]
    pub async fn records(db: DatabaseConnection) -> ServerResult<HarvestSubscriptionList> {
        match sqlx::query!(
            r#"
                SELECT subscription.id,
                    subscription.harvest_id,
                    subscription.amount,
                    subscription.expires_at,
                    subscription.created_at
                FROM features.harvest_subscriptions subscription
            "#
        )
        .fetch_all(&db.pool)
        .await
        {
            Ok(records) => {
                let regions = records
                    .into_iter()
                    .map(|rec| {
                        Self::from_row(
                            rec.id.into(),
                            rec.harvest_id.into(),
                            rec.amount,
                            rec.expires_at,
                            rec.created_at,
                        )
                    })
                    .collect();

                Ok(regions)
            }
            Err(err) => {
                tracing::error!(
                    "Database error, failed to fetch harvest subscription: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Inserts harvest subscription into the database
    #[tracing::instrument(name = "Insert Location-region", skip(db, subscription))]
    pub async fn insert(
        subscription: HarvestSubscriptionInsertData,
        db: DatabaseConnection,
    ) -> ServerResult<ModelID> {
        match sqlx::query!(
            r#"
                INSERT INTO features.harvest_subscriptions (
                    id,
                    harvest_id,
                    amount,
                    expires_at,
                    created_at
                )
                VALUES ($1, $2, $3, $4, $5);
            "#,
            subscription.id.0,
            subscription.harvest_id.0,
            subscription.amount,
            subscription.expires_at,
            subscription.created_at
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Harvest subscription inserted successfully: {:?}", result);
                Ok(subscription.id)
            }
            Err(err) => {
                // Handle database constraint error
                handle_harvest_subscription_database_error(&err)?;

                tracing::error!(
                    "Database error, failed to insert harvest subscription: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Updates Harvest subscription in the database
    #[tracing::instrument(name = "Update Harvest Subscription", skip(db, subscription))]
    pub async fn update(
        id: ModelID,
        subscription: HarvestSubscriptionUpdateData,
        db: DatabaseConnection,
    ) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                UPDATE features.harvest_subscriptions subscription
                SET harvest_id = COALESCE($1, subscription.harvest_id),
                    amount = COALESCE($2, subscription.amount),
                    expires_at = COALESCE($3, subscription.expires_at)
                WHERE subscription.id = $4
           "#,
            subscription.harvest_id.0,
            subscription.amount,
            subscription.expires_at,
            id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Harvest subscription updated successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_harvest_subscription_database_error(&err)?;

                tracing::error!(
                    "Database error, failed to update harvest subscription: {}",
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Deletes harvest subscription from the database
    #[tracing::instrument(name = "Delete Harvest Subscription", skip(db))]
    pub async fn delete(id: ModelID, db: DatabaseConnection) -> ServerResult<()> {
        match sqlx::query!(
            r#"
                DELETE FROM features.harvest_subscriptions subscription
                WHERE subscription.id = $1
           "#,
            id.0
        )
        .execute(&db.pool)
        .await
        {
            Ok(result) => {
                tracing::debug!("Harvest subscription deleted successfully: {:?}", result);
                Ok(())
            }
            Err(err) => {
                // Handle database constraint error
                handle_harvest_subscription_database_error(&err)?;

                tracing::error!(
                    "Database error, failed to delete harvest subscription: {}",
                    err
                );
                Err(err.into())
            }
        }
    }
}

/// Handle regions database constraints errors
#[allow(clippy::cognitive_complexity)]
fn handle_harvest_subscription_database_error(err: &sqlx::Error) -> ServerResult<()> {
    if let sqlx::Error::Database(db_err) = err {
        // Handle db unique constraints
        if db_err.is_unique_violation() {
            tracing::error!("Database error, Harvest subscribed already. {:?}", err);
            return Err(ServerError::rejection(EndpointRejection::Conflict(
                "Harvest subscription already exists.".into(),
            )));
        }

        // Handle db foreign key constraints
        if db_err.is_foreign_key_violation() {
            tracing::error!("Database error, harvest not found. {:?}", err);
            return Err(ServerError::rejection(EndpointRejection::BadRequest(
                "Harvest  not found.".into(),
            )));
        }
    }

    if matches!(err, &sqlx::Error::RowNotFound) {
        tracing::error!("Database error, harvest subscription not found. {:?}", err);
        return Err(ServerError::rejection(EndpointRejection::NotFound(
            "Harvest subscription not found.".into(),
        )));
    }

    Ok(())
}
