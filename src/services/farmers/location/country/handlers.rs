//! Location country http handlers impls

use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use uuid::Uuid;

use crate::{
    auth::CurrentUser,
    endpoint::{EndpointRejection, EndpointResult, ModelId, ValidatedJson},
    server::state::DatabaseConnection,
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
#[tracing::instrument(skip(db))]
pub async fn country_create(
    current_user: CurrentUser,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<CountryForm>,
) -> EndpointResult<StatusCode> {
    if current_user.is_staff {
        Country::insert(form.into(), db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_id| Ok(StatusCode::CREATED),
        )
    } else {
        Err(EndpointRejection::forbidden())
    }
}

/// Handles the `PUT /locations/countries/:country_id` route.
#[tracing::instrument(skip(db))]
pub async fn country_update(
    current_user: CurrentUser,
    ModelId(country_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
    ValidatedJson(form): ValidatedJson<CountryForm>,
) -> EndpointResult<StatusCode> {
    if current_user.is_staff {
        Country::update(country_id, form.into(), db)
            .await
            .map_or_else(
                |_err| Err(EndpointRejection::internal_server_error()),
                |_| Ok(StatusCode::OK),
            )
    } else {
        Err(EndpointRejection::forbidden())
    }
}

/// Handles the `DELETE /locations/countries/:country_id` route.
#[tracing::instrument(skip(db))]
pub async fn country_delete(
    current_user: CurrentUser,
    ModelId(country_id): ModelId<Uuid>,
    State(db): State<DatabaseConnection>,
) -> EndpointResult<StatusCode> {
    if current_user.is_staff {
        Country::delete(country_id, db).await.map_or_else(
            |_err| Err(EndpointRejection::internal_server_error()),
            |_| Ok(StatusCode::NO_CONTENT),
        )
    } else {
        Err(EndpointRejection::forbidden())
    }
}
