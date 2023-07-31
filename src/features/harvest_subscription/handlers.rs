//! `HarvestSubscription` http handlers impls

use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use crate::{
    auth::{AdminUser, CurrentUser, SuperUser},
    endpoint::EndpointResult,
    server::state::DatabaseConnection,
    types::ModelID,
};

use super::{
    forms::HarvestSubscriptionForm,
    models::{HarvestSubscription, HarvestSubscriptionList},
};

/// Handles the `GET /harvests/subscription` route.
#[tracing::instrument(skip(db))]
pub async fn harvest_subscription_list(
    _: AdminUser,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<HarvestSubscriptionList>> {
    let subscriptions = HarvestSubscription::records(db).await?;
    Ok(Json(subscriptions))
}

/// Handles the `GET /account/harvests-subscriptions` route.
#[tracing::instrument(skip(db, user))]
pub async fn user_harvest_subscriptions(
    user: CurrentUser,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<HarvestSubscriptionList>> {
    let subscriptions = HarvestSubscription::user_records(user.id, db).await?;
    Ok(Json(subscriptions))
}

/// Handles the `POST /harvests/subscription` route.
#[tracing::instrument(skip(db, form))]
pub async fn harvest_subscription_create(
    _: SuperUser,
    State(db): State<DatabaseConnection>,
    form: HarvestSubscriptionForm,
) -> EndpointResult<StatusCode> {
    HarvestSubscription::insert(form.into(), db).await?;
    Ok(StatusCode::CREATED)
}

/// Handles the `PUT /harvests/subscription` route.
#[tracing::instrument(skip(db, form))]
pub async fn harvest_subscription_update(
    _: SuperUser,
    subscription_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: HarvestSubscriptionForm,
) -> EndpointResult<StatusCode> {
    HarvestSubscription::update(subscription_id, form.into(), db).await?;
    Ok(StatusCode::OK)
}

/// Handles the `DELETE /harvests/subscription` route.
#[tracing::instrument(skip(db))]
pub async fn harvest_subscription_delete(
    _: SuperUser,
    subscription_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    HarvestSubscription::delete(subscription_id, db).await?;
    Ok(StatusCode::NO_CONTENT)
}
