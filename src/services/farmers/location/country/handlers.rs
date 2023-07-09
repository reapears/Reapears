//! Location country http handlers impls

use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use crate::{
    auth::AdminUser,
    endpoint::{EndpointRejection, EndpointResult},
    server::state::DatabaseConnection,
    types::ModelID,
};

use super::{forms::CountryForm, Country, CountryList};

/// Handles the `GET /locations/countries` route.
#[tracing::instrument(skip(db))]
pub async fn country_list(
    State(db): State<DatabaseConnection>,
) -> EndpointResult<Json<CountryList>> {
    Country::records(db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |countries| Ok(Json(countries)),
    )
}

/// Handles the `POST /locations/countries` route.
#[tracing::instrument(skip(db, user, form))]
pub async fn country_create(
    #[allow(unused_variables)] user: AdminUser,
    State(db): State<DatabaseConnection>,
    form: CountryForm,
) -> EndpointResult<StatusCode> {
    Country::insert(form.into(), db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |_id| Ok(StatusCode::CREATED),
    )
}

/// Handles the `PUT /locations/countries/:country_id` route.
#[tracing::instrument(skip(db, user, form))]
pub async fn country_update(
    #[allow(unused_variables)] user: AdminUser,
    country_id: ModelID,
    State(db): State<DatabaseConnection>,
    form: CountryForm,
) -> EndpointResult<StatusCode> {
    Country::update(country_id, form.into(), db)
        .await
        .map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::OK),
        )
}

/// Handles the `DELETE /locations/countries/:country_id` route.
#[tracing::instrument(skip(db, user))]
pub async fn country_delete(
    #[allow(unused_variables)] user: AdminUser,
    country_id: ModelID,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    Country::delete(country_id, db).await.map_or_else(
        |_err| Err(EndpointRejection::internal_server_error()),
        |_| Ok(StatusCode::NO_CONTENT),
    )
}
