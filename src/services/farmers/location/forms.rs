//! Location forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, FromRequestParts, Json, Request},
};
use geo::Point;
use serde::Deserialize;
use time::{Date, OffsetDateTime};

use crate::{
    auth::FarmerUser,
    endpoint::{
        validators::{TransformString, ValidateString},
        EndpointRejection, EndpointResult,
    },
    server::state::ServerState,
    services::farmers::farm::permissions::check_user_owns_farm,
    types::ModelID,
};

use super::permissions::check_user_owns_location;

/// Embedded location create form,
/// this form is embedded in `FarmCreateForm`.
/// It differs from `LocationCreateForm` that
/// it does not validate `farm_id` because on farm creation
/// the farm does not exists yet
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationEmbeddedForm {
    pub place_name: String,
    pub region_id: String,
    pub country_id: String,
    pub description: Option<String>,
    pub coords: Option<Point>,
}

impl LocationEmbeddedForm {
    /// Validates embedded location form inputs
    pub fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.country_id.validate_id("Invalid country id")?;
        self.region_id.validate_id("Invalid region id")?;

        self.place_name
            .validate_len(0, 64, "Place name must be at most 64 characters")?;

        if let Some(ref desc) = self.description {
            desc.validate_len(
                0,
                512,
                "Location description must be at most 512 characters",
            )?;
        }

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.place_name = self.place_name.clean().to_titlecase();
        self.description = self.description.as_ref().map(|desc| desc.clean());
    }

    /// Converts `Self` into `LocationInsertData`
    #[allow(dead_code)]
    #[must_use]
    pub fn data(self, farm_id: ModelID) -> LocationInsertData {
        LocationInsertData {
            id: ModelID::new(),
            farm_id,
            place_name: self.place_name,
            region_id: ModelID::from_str_unchecked(&self.region_id),
            country_id: ModelID::from_str_unchecked(&self.country_id),
            description: self.description,
            coords: serde_json::to_value(self.coords).ok(),
            created_at: OffsetDateTime::now_utc().date(),
        }
    }
}

// ===== Location Create form impl =====

/// Location create form, this form is used
/// when you want to add a new `Farm` location
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationCreateForm {
    pub place_name: String,
    pub region_id: String,
    pub country_id: String,
    pub description: Option<String>,
    pub coords: Option<Point>,
}

/// Location create cleaned data
#[derive(Debug, Clone)]
pub struct LocationInsertData {
    pub id: ModelID,
    pub farm_id: ModelID,
    pub place_name: String,
    pub region_id: ModelID,
    pub country_id: ModelID,
    pub description: Option<String>,
    pub coords: Option<serde_json::Value>,
    pub created_at: Date,
}

impl LocationCreateForm {
    /// Validates location form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.country_id.validate_id("Invalid country id")?;
        self.region_id.validate_id("Invalid region id")?;

        self.place_name
            .validate_len(0, 64, "Place name must be at most 64 characters")?;

        if let Some(ref desc) = self.description {
            desc.validate_len(
                0,
                512,
                "Location description must be at most 512 characters",
            )?;
        }

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.place_name = self.place_name.clean().to_titlecase();
        self.description = self.description.as_ref().map(|desc| desc.clean());
    }

    /// Convert `Self` into `LocationInsertData`
    #[allow(dead_code)]
    #[must_use]
    pub fn data(self, farm_id: ModelID) -> LocationInsertData {
        LocationInsertData {
            id: ModelID::new(),
            farm_id,
            place_name: self.place_name,
            region_id: ModelID::from_str_unchecked(&self.region_id),
            country_id: ModelID::from_str_unchecked(&self.country_id),
            description: self.description,
            coords: serde_json::to_value(self.coords).ok(),
            created_at: OffsetDateTime::now_utc().date(),
        }
    }

    ///  Validate a user has the permissions to crate a location on this farm
    async fn authorize_request(
        user: FarmerUser,
        farm_id: ModelID,
        state: &ServerState,
    ) -> EndpointResult<()> {
        check_user_owns_farm(user.id(), farm_id, state.database()).await
    }
}

#[async_trait]
impl FromRequest<ServerState> for LocationCreateForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let (mut parts, body) = req.into_parts();
        let user = { FarmerUser::from_parts(&mut parts, state).await? };
        let farm_id = { ModelID::from_request_parts(&mut parts, state).await? };
        let Json(mut location) =
            Json::<Self>::from_request(Request::from_parts(parts, body), state).await?;

        // Validate from fields
        location.validate()?;

        // Authorize the request
        Self::authorize_request(user, farm_id, state).await?;

        Ok(location)
    }
}

// ===== Location Update form impls ======

/// Location update form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationUpdateForm {
    pub place_name: String,
    pub region_id: String,
    pub country_id: String,
    pub description: Option<String>,
    pub coords: Option<Point>,
}

/// Location update form cleaned data
#[derive(Debug, Clone)]
pub struct LocationUpdateData {
    pub place_name: String,
    pub region_id: ModelID,
    pub country_id: ModelID,
    pub description: Option<String>,
    pub coords: Option<serde_json::Value>,
}

impl From<LocationUpdateForm> for LocationUpdateData {
    fn from(form: LocationUpdateForm) -> Self {
        Self {
            place_name: form.place_name,
            region_id: ModelID::from_str_unchecked(form.region_id),
            country_id: ModelID::from_str_unchecked(form.country_id),
            description: form.description,
            coords: serde_json::to_value(form.coords).ok(),
        }
    }
}

impl LocationUpdateForm {
    /// Validates location form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.country_id.validate_id("Invalid country id")?;
        self.region_id.validate_id("Invalid region id")?;

        self.place_name
            .validate_len(0, 64, "Place name must be at most 64 characters")?;

        if let Some(ref desc) = self.description {
            desc.validate_len(
                0,
                512,
                "Location description must be at most 512 characters",
            )?;
        }

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.place_name = self.place_name.clean().to_titlecase();
        self.description = self.description.as_ref().map(|desc| desc.clean());
    }

    ///  Validate a user has the permissions to crate a location on this farm
    async fn authorize_request(
        user: FarmerUser,
        location_id: ModelID,
        state: &ServerState,
    ) -> EndpointResult<()> {
        check_user_owns_location(user.id(), location_id, state.database()).await
    }
}

#[async_trait]
impl FromRequest<ServerState> for LocationUpdateForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let (mut parts, body) = req.into_parts();
        let user = { FarmerUser::from_parts(&mut parts, state).await? };
        let location_id = { ModelID::from_request_parts(&mut parts, state).await? };
        let Json(mut location) =
            Json::<Self>::from_request(Request::from_parts(parts, body), state).await?;

        // Validate form fields
        location.validate()?;

        // Authorize the request
        Self::authorize_request(user, location_id, state).await?;

        Ok(location)
    }
}
