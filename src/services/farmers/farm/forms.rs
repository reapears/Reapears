//! Farm forms impls

use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, FromRequestParts, Json, Request},
};
use serde::Deserialize;
use time::{Date, OffsetDateTime};

use crate::{
    auth::FarmerUser,
    endpoint::{
        validators::{TransformString, ValidateString},
        EndpointRejection, EndpointResult,
    },
    server::state::ServerState,
    services::farmers::location::forms::{LocationEmbeddedForm, LocationInsertData},
    types::ModelID,
};

use super::permissions::check_user_owns_farm;

/// Farm create form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FarmCreateForm {
    pub name: String,
    pub contact_email: Option<String>,
    pub contact_number: Option<String>,
    pub founded_at: Option<Date>,
    pub location: LocationEmbeddedForm,
}

/// Farm create cleaned data
#[derive(Debug, Clone)]
pub struct FarmInsertData {
    pub id: ModelID,
    pub owner_id: ModelID,
    pub name: String,
    pub contact_email: Option<String>,
    pub contact_number: Option<String>,
    pub founded_at: Option<Date>,
    pub location: LocationInsertData,
    pub registered_on: Date,
}

impl FarmCreateForm {
    /// Validates farm form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.name
            .validate_len(0, 32, "Farm name must be at most 128 characters")?;

        if let Some(ref email) = self.contact_email {
            email.validate_email()?;
        }

        if let Some(ref phone) = self.contact_number {
            self.contact_number = Some(phone.validate_phone()?);
        }

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.name = self.name.clean().to_titlecase();
        self.contact_email = self
            .contact_email
            .as_ref()
            .map(|email| email.clean().to_ascii_lowercase());
    }

    /// Convert `Self` into `FarmInsertData`
    #[allow(dead_code)]
    #[must_use]
    pub fn data(self, user_id: ModelID) -> FarmInsertData {
        let id = ModelID::new();
        FarmInsertData {
            id,
            owner_id: user_id,
            name: self.name,
            contact_email: self.contact_email,
            contact_number: self.contact_number,
            founded_at: self.founded_at,
            location: self.location.data(id),
            registered_on: OffsetDateTime::now_utc().date(),
        }
    }
}

#[async_trait]
impl FromRequest<ServerState> for FarmCreateForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let Json(mut farm) = Json::<Self>::from_request(req, state).await?;

        // Validate form fields
        farm.validate()?;
        farm.location.validate()?;

        Ok(farm)
    }
}

// ===== Farm Update form impls ======

/// Farm update form
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FarmUpdateForm {
    pub name: String,
    pub contact_email: Option<String>,
    pub contact_number: Option<String>,
    pub founded_at: Option<Date>,
}

/// Farm update cleaned data
#[derive(Debug, Clone)]
pub struct FarmUpdateData {
    pub name: String,
    pub contact_email: Option<String>,
    pub contact_number: Option<String>,
    pub founded_at: Option<Date>,
}

impl From<FarmUpdateForm> for FarmUpdateData {
    fn from(form: FarmUpdateForm) -> Self {
        Self {
            name: form.name,
            contact_number: form.contact_number,
            contact_email: form.contact_email,
            founded_at: form.founded_at,
        }
    }
}

impl FarmUpdateForm {
    /// Validates region form inputs
    fn validate(&mut self) -> EndpointResult<()> {
        // Clean the data
        self.clean_data();

        self.name
            .validate_len(0, 32, "Farm name must be at most 128 characters")?;

        if let Some(ref email) = self.contact_email {
            email.validate_email()?;
        }

        if let Some(ref phone) = self.contact_number {
            self.contact_number = Some(phone.validate_phone()?);
        }

        Ok(())
    }

    /// Clean form data
    fn clean_data(&mut self) {
        self.name = self.name.clean().to_titlecase();
        self.contact_email = self
            .contact_email
            .as_ref()
            .map(|email| email.clean().to_ascii_lowercase());
    }

    ///  Validate a user has the permissions to update a farm
    async fn authorize_request(
        state: &ServerState,
        user: FarmerUser,
        farm_id: ModelID,
    ) -> EndpointResult<()> {
        check_user_owns_farm(user.id(), farm_id, state.database()).await
    }
}

#[async_trait]
impl FromRequest<ServerState> for FarmUpdateForm
where
    Json<Self>: FromRequest<ServerState, Rejection = JsonRejection>,
{
    type Rejection = EndpointRejection;

    async fn from_request(req: Request, state: &ServerState) -> Result<Self, Self::Rejection> {
        // Extract data
        let (mut parts, body) = req.into_parts();
        let user = { FarmerUser::from_parts(&mut parts, state).await? };
        let farm_id = { ModelID::from_request_parts(&mut parts, state).await? };
        let Json(mut farm) =
            Json::<Self>::from_request(Request::from_parts(parts, body), state).await?;

        // Validate form fields
        farm.validate()?;

        // Authorize request
        Self::authorize_request(state, user, farm_id).await?;

        Ok(farm)
    }
}
