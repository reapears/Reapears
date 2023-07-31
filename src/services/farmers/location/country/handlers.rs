//! Location country http handlers impls

use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use crate::{
    auth::AdminUser, endpoint::EndpointResult, server::state::DatabaseConnection, types::ModelID,
};

use super::{forms::CountryForm, Country, CountryList};

/// Handles the `GET /locations/countries` route.
#[tracing::instrument(skip(db))]
pub async fn country_list(
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<CountryList>> {
    let countries = Country::records(db).await?;
    Ok(Json(countries))
}

/// Handles the `POST /locations/countries` route.
#[tracing::instrument(skip(db, form))]
pub async fn country_create(
    _: AdminUser,
    State(db): State<DatabaseConnection>,
    form: CountryForm,
) -> EndpointResult<StatusCode> {
    Country::insert(form.into(), db).await?;
    Ok(StatusCode::CREATED)
}

/// Handles the `PUT /locations/countries/:country_id` route.
#[tracing::instrument(skip(db, form))]
pub async fn country_update(
    _: AdminUser,
    country_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: CountryForm,
) -> EndpointResult<StatusCode> {
    Country::update(country_id, form.into(), db).await?;
    Ok(StatusCode::OK)
}

/// Handles the `DELETE /locations/countries/:country_id` route.
#[tracing::instrument(skip(db,))]
pub async fn country_delete(
    _: AdminUser,
    country_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    Country::delete(country_id, db).await?;
    Ok(StatusCode::NO_CONTENT)
}
