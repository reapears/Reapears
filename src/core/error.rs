//! Server error impls

use std::{error::Error as StdError, fmt};

use crate::endpoint::EndpointRejection;

pub type ServerResult<T> = Result<T, ServerError>;
type GenericError = Box<dyn StdError + Send + Sync + 'static>;

/// Error that can happen on the server
#[derive(Debug)]
pub struct ServerError {
    pub kind: ServerErrorKind,
}

impl ServerError {
    pub const MESSAGE: &'static str = "\
Something went wrong Unfortunately, a server error prevented your request from being completed. Reapears may be undergoing maintenance or your connection may have timed out. Please refresh the page or try again.";

    /// Create a new internal server with message
    pub fn new(error_msg: impl AsRef<str>) -> Self {
        let kind =
            ServerErrorKind::Internal(Box::<dyn StdError + Send + Sync>::from(error_msg.as_ref()));
        Self { kind }
    }

    /// Return whether the error is coursed by the user
    #[must_use]
    pub const fn is_user_error(&self) -> bool {
        matches!(self.kind, ServerErrorKind::UserError(_))
    }

    /// Create a new user error
    /// with the status code of bad request
    #[must_use]
    pub fn bad_request(err_msg: impl AsRef<str>) -> Self {
        let err_msg = err_msg.as_ref().to_owned();
        let kind = ServerErrorKind::UserError(EndpointRejection::BadRequest(err_msg.into()));
        Self { kind }
    }

    /// Create a new `EndpointRejection` error
    #[must_use]
    pub fn rejection(rej: EndpointRejection) -> Self {
        Self {
            kind: ServerErrorKind::UserError(rej),
        }
    }

    /// Create a new internal server from a generic error
    #[must_use]
    pub fn internal(err: GenericError) -> Self {
        Self {
            kind: ServerErrorKind::Internal(err),
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl StdError for ServerError {}

/// Error kind of `ServerError` that can happen on the server.
#[derive(Debug)]
pub enum ServerErrorKind {
    /// An error coursed by a client
    UserError(EndpointRejection),
    /// A wrapper on top of `sqlx::Error`
    Database(sqlx::Error),
    /// A wrapper on top of `serde_json::Error`
    JsonError(serde_json::Error),
    /// A wrapper on top of `password_hash::Error`
    PasswordHash(password_auth::VerifyError),
    /// A wrapper on top of `image::ImageError`
    ImageError(image::ImageError),
    /// A wrapper on top of `lettre::error::Error`
    LettreError(lettre::error::Error),
    /// A wrapper on top of `std::io::Error`
    Io(std::io::Error),
    /// Error returned when a server encountered an error it does not want to handle.
    Internal(GenericError),
}

impl fmt::Display for ServerErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserError(err) => err.fmt(f),
            Self::Database(err) => err.fmt(f),
            Self::Io(err) => err.fmt(f),
            Self::PasswordHash(err) => err.fmt(f),
            Self::JsonError(err) => err.fmt(f),
            Self::ImageError(err) => err.fmt(f),
            Self::LettreError(err) => err.fmt(f),
            Self::Internal(err) => err.fmt(f),
        }
    }
}

impl From<ServerErrorKind> for ServerError {
    fn from(kind: ServerErrorKind) -> Self {
        Self { kind }
    }
}

impl From<sqlx::Error> for ServerError {
    fn from(err: sqlx::Error) -> Self {
        Self {
            kind: ServerErrorKind::Database(err),
        }
    }
}

impl From<serde_json::Error> for ServerError {
    fn from(err: serde_json::Error) -> Self {
        Self {
            kind: ServerErrorKind::JsonError(err),
        }
    }
}

impl From<image::ImageError> for ServerError {
    fn from(err: image::ImageError) -> Self {
        Self {
            kind: ServerErrorKind::ImageError(err),
        }
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        Self {
            kind: ServerErrorKind::Io(err),
        }
    }
}

impl From<GenericError> for ServerError {
    fn from(err: GenericError) -> Self {
        Self {
            kind: ServerErrorKind::Internal(err),
        }
    }
}

impl From<password_auth::VerifyError> for ServerError {
    fn from(err: password_auth::VerifyError) -> Self {
        Self {
            kind: ServerErrorKind::PasswordHash(err),
        }
    }
}

impl From<tokio::task::JoinError> for ServerError {
    fn from(err: tokio::task::JoinError) -> Self {
        Self::internal(Box::new(err))
    }
}

impl From<lettre::error::Error> for ServerError {
    fn from(err: lettre::error::Error) -> Self {
        Self {
            kind: ServerErrorKind::LettreError(err),
        }
    }
}
