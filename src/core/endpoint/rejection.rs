//! Response rejection impls

#![allow(clippy::enum_glob_use)]

use axum::{
    extract::rejection::JsonRejection,
    // headers,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
    // TypedHeader,
};
use axum_extra::extract::PrivateCookieJar;
use serde::Serialize;
use serde_json::{json, Value};

use std::fmt;

use crate::error::ServerError;
use EndpointRejection::*;

pub type EndpointResult<T> = Result<T, EndpointRejection>;

// pub type LocationHeader = (&static str, String);

// /// Contract a location header
// pub fn LocationHeader(url: &str) -> TypedHeader<headers::Location> {
//     TypedHeader(headers::Location(headers::HeaderValue::from_static(url)))
// }

/// A response returned by handlers
/// when could not get intended response
#[derive(Debug)]
pub enum EndpointRejection {
    // Client error responses
    BadRequest(RejectionResponse),                  // 400
    Unauthorized(RejectionResponse),                // 401
    Forbidden(RejectionResponse),                   // 403
    NotFound(RejectionResponse),                    // 404
    MethodNotAllowed(RejectionResponse),            // 405
    NotAcceptable(RejectionResponse),               // 406
    RequestTimeout(RejectionResponse),              // 408
    Conflict(RejectionResponse),                    // 409
    LengthRequired(RejectionResponse),              // 411
    PayloadTooLarge(RejectionResponse),             // 413
    UnsupportedMediaType(RejectionResponse),        // 415
    UpgradeRequired(RejectionResponse),             // 406
    UnprocessableEntity(RejectionResponse),         // 422
    TooManyRequests(RejectionResponse),             // 429
    UnavailableForLegalReasons(RejectionResponse),  // 451
    RequestHeaderFieldsTooLarge(RejectionResponse), // 431

    //Server error responses
    InternalServerError(RejectionResponse),     // 500
    NotImplemented(RejectionResponse),          // 501
    ServiceUnavailable(RejectionResponse),      // 503
    HTTPVersionNotSupported(RejectionResponse), // 505
}

impl std::error::Error for EndpointRejection {}

impl fmt::Display for EndpointRejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl EndpointRejection {
    /// Return an error response with a status code of `InternalServeError`
    #[must_use]
    pub fn internal_server_error() -> Self {
        Self::InternalServerError(ServerError::MESSAGE.into())
    }

    /// Return an error response with a status code of `NotFound`
    pub fn not_found(item: impl fmt::Display) -> Self {
        let item = item.to_string().to_lowercase();
        Self::NotFound(
            format!(" Oops! We can't seem to find the {item} you're looking for.").into(),
        )
    }

    /// Return an error response with a status code of `Unauthorized`
    #[must_use]
    pub fn unauthorized() -> Self {
        Self::Unauthorized("Unauthorized! Server failed to authenticate the request.".into())
    }

    /// Return an error response with a status code of `Unauthorized`
    ///
    /// For security reasons we are only returning `Self::BadRequest`
    #[must_use]
    pub fn forbidden() -> Self {
        Self::BadRequest("Server could not process the request.".into())
    }
}

impl From<ServerError> for EndpointRejection {
    fn from(err: ServerError) -> Self {
        if err.is_user_error() {
            Self::BadRequest(err.to_string().into())
        } else {
            Self::InternalServerError(ServerError::MESSAGE.into())
        }
    }
}

impl IntoResponse for EndpointRejection {
    fn into_response(self) -> Response {
        let (status, rejection_response) = match self {
            // Client Errors
            BadRequest(response) => (StatusCode::BAD_REQUEST, response),
            Unauthorized(response) => (StatusCode::UNAUTHORIZED, response),
            Forbidden(response) => (StatusCode::FORBIDDEN, response),
            NotFound(response) => (StatusCode::NOT_FOUND, response),
            MethodNotAllowed(response) => (StatusCode::METHOD_NOT_ALLOWED, response),
            NotAcceptable(response) => (StatusCode::NOT_ACCEPTABLE, response),
            RequestTimeout(response) => (StatusCode::REQUEST_TIMEOUT, response),
            Conflict(response) => (StatusCode::CONFLICT, response),
            LengthRequired(response) => (StatusCode::LENGTH_REQUIRED, response),
            PayloadTooLarge(response) => (StatusCode::PAYLOAD_TOO_LARGE, response),
            UnsupportedMediaType(response) => (StatusCode::UNSUPPORTED_MEDIA_TYPE, response),
            UpgradeRequired(response) => (StatusCode::UPGRADE_REQUIRED, response),
            UnprocessableEntity(response) => (StatusCode::UNPROCESSABLE_ENTITY, response),
            TooManyRequests(response) => (StatusCode::TOO_MANY_REQUESTS, response),
            UnavailableForLegalReasons(response) => {
                (StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS, response)
            }
            RequestHeaderFieldsTooLarge(response) => {
                (StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE, response)
            }
            //Server Errors
            InternalServerError(response) => (StatusCode::INTERNAL_SERVER_ERROR, response),
            NotImplemented(response) => (StatusCode::NOT_IMPLEMENTED, response),
            ServiceUnavailable(response) => (StatusCode::SERVICE_UNAVAILABLE, response),
            HTTPVersionNotSupported(response) => (StatusCode::HTTP_VERSION_NOT_SUPPORTED, response),
        };

        let mut response = rejection_response.0;
        *response.status_mut() = status;
        response
    }
}

impl From<JsonRejection> for EndpointRejection {
    fn from(err: JsonRejection) -> Self {
        let error_msg = "Unable to deserialize request";
        match err {
            JsonRejection::JsonDataError(reason) => {
                Self::UnprocessableEntity(format!("{error_msg}: {reason}").into())
            }
            JsonRejection::JsonSyntaxError(reason) => {
                Self::BadRequest(format!("{error_msg}: {reason}").into())
            }
            JsonRejection::MissingJsonContentType(reason) => {
                Self::UnsupportedMediaType(format!("{error_msg}: {reason}").into())
            }
            _ => {
                Self::InternalServerError(format!("{}: {}", error_msg, ServerError::MESSAGE).into())
            }
        }
    }
}

// RejectionResponse
pub trait IntoRejectionResponse {
    fn into_rejection_response(self) -> RejectionResponse;
}

#[derive(Debug)]
pub struct RejectionResponse(Response);

impl<T> From<T> for RejectionResponse
where
    T: IntoRejectionResponse,
{
    fn from(res: T) -> Self {
        res.into_rejection_response()
    }
}

impl IntoRejectionResponse for ServerError {
    fn into_rejection_response(self) -> RejectionResponse {
        let rej_body = Self::MESSAGE.into_rejection_body();
        RejectionResponse(rej_body.into_response())
    }
}

impl IntoRejectionResponse for &'static str {
    fn into_rejection_response(self) -> RejectionResponse {
        let rej_body = self.into_rejection_body();
        RejectionResponse(rej_body.into_response())
    }
}

impl IntoRejectionResponse for String {
    fn into_rejection_response(self) -> RejectionResponse {
        let body = self.into_rejection_body();
        RejectionResponse(body.into_response())
    }
}

impl<K, B> IntoRejectionResponse for (B, PrivateCookieJar<K>)
where
    B: IntoRejectionBody,
{
    fn into_rejection_response(self) -> RejectionResponse {
        let (body, cookies) = self;
        let body = body.into_rejection_body();
        RejectionResponse((cookies, body).into_response())
    }
}

/// `IntoRejectionBody`
/// converts `T` an error message into a json body for `RejectionResponse`
pub trait IntoRejectionBody: Serialize {
    fn into_rejection_body(self) -> Json<Value>
    where
        Self: Sized,
    {
        Json(json!({"error": {"message": self}}))
    }
}

impl<T> IntoRejectionBody for T where T: Serialize + fmt::Display + Send + Sync {}
