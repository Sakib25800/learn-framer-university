//! This module implements several error types and traits.
//!
//! * `util::errors::AppResult` - Failures can be converted into user facing JSON responses. This error type
//!   is more dynamic is box allocated. Low-level errors are typically not converted to user facing errors and most usage
//!   lies within models, controllers, and middleware layers.

use std::{
    any::{Any, TypeId},
    borrow::Cow,
    error::Error,
    fmt,
};

use axum::{response::IntoResponse, Extension};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use http::StatusCode;
use json::custom;
use tokio::task::JoinError;
use validator::ValidationErrors;

use crate::middleware::log_request::ErrorField;

mod json;

pub type BoxedAppError = Box<dyn AppError>;

pub fn bad_request(detail: impl Into<Cow<'static, str>>) -> BoxedAppError {
    custom(StatusCode::BAD_REQUEST, detail)
}

pub fn forbidden(detail: impl Into<Cow<'static, str>>) -> BoxedAppError {
    custom(StatusCode::FORBIDDEN, detail)
}

pub fn auth(detail: impl Into<Cow<'static, str>>) -> BoxedAppError {
    custom(StatusCode::UNAUTHORIZED, detail)
}

pub fn not_found(detail: impl Into<Cow<'static, str>>) -> BoxedAppError {
    custom(StatusCode::NOT_FOUND, detail)
}

/// Returns an error with status 503 and the provided description as JSON
pub fn service_unavailable() -> BoxedAppError {
    custom(StatusCode::SERVICE_UNAVAILABLE, "Service unavailable")
}

// =============================================================================
// AppError trait

pub trait AppError: Send + fmt::Display + fmt::Debug + 'static {
    /// Generate an HTTP response for the error
    ///
    /// If none is returned, the error will bublbe up the middleware stack
    /// where it is eventually logged and a status 500.
    fn response(&self) -> axum::response::Response;

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

impl dyn AppError {
    pub fn is<T: Any>(&self) -> bool {
        self.get_type_id() == TypeId::of::<T>()
    }
}

impl AppError for BoxedAppError {
    fn response(&self) -> axum::response::Response {
        (**self).response()
    }

    fn get_type_id(&self) -> TypeId {
        (**self).get_type_id()
    }
}

impl IntoResponse for BoxedAppError {
    fn into_response(self) -> axum::response::Response {
        self.response()
    }
}

pub type AppResult<T> = Result<T, BoxedAppError>;

// =============================================================================
// Error impls

impl<E: Error + Send + 'static> AppError for E {
    fn response(&self) -> axum::response::Response {
        error!(error = %self, "Internal Server Error");

        sentry::capture_error(self);

        server_error_response(self.to_string())
    }
}

impl From<diesel::ConnectionError> for BoxedAppError {
    fn from(err: diesel::ConnectionError) -> BoxedAppError {
        Box::new(err)
    }
}

impl From<DieselError> for BoxedAppError {
    fn from(err: DieselError) -> BoxedAppError {
        match err {
            DieselError::NotFound => not_found(""),
            DieselError::DatabaseError(DatabaseErrorKind::ClosedConnection, _) => {
                service_unavailable()
            }
            _ => Box::new(err),
        }
    }
}

impl From<diesel_async::pooled_connection::deadpool::PoolError> for BoxedAppError {
    fn from(err: diesel_async::pooled_connection::deadpool::PoolError) -> BoxedAppError {
        error!("Database pool error: {err}");
        service_unavailable()
    }
}

impl From<prometheus::Error> for BoxedAppError {
    fn from(err: prometheus::Error) -> BoxedAppError {
        Box::new(err)
    }
}

impl From<serde_json::Error> for BoxedAppError {
    fn from(err: serde_json::Error) -> BoxedAppError {
        Box::new(err)
    }
}

impl From<std::io::Error> for BoxedAppError {
    fn from(err: std::io::Error) -> BoxedAppError {
        Box::new(err)
    }
}

impl From<JoinError> for BoxedAppError {
    fn from(err: JoinError) -> BoxedAppError {
        Box::new(err)
    }
}

impl From<ValidationErrors> for BoxedAppError {
    fn from(err: ValidationErrors) -> BoxedAppError {
        Box::new(err)
    }
}

// =============================================================================
// Internal error for use with `chain_error`

#[derive(Debug)]
struct InternalAppError {
    description: String,
}

impl fmt::Display for InternalAppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)?;
        Ok(())
    }
}

impl AppError for InternalAppError {
    fn response(&self) -> axum::response::Response {
        error!(error = %self.description, "Internal Server Error");

        sentry::capture_message(&self.description, sentry::Level::Error);

        server_error_response(self.description.to_string())
    }
}

pub fn internal<S: ToString>(error: S) -> BoxedAppError {
    Box::new(InternalAppError {
        description: error.to_string(),
    })
}

fn server_error_response(error: String) -> axum::response::Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Extension(ErrorField(error)),
        "Internal Server Error",
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::result::Error as DieselError;
    use http::StatusCode;

    #[test]
    fn http_error_responses() {
        use crate::serde::de::Error;

        // Types for handling common error status codes
        assert_eq!(bad_request("").response().status(), StatusCode::BAD_REQUEST);
        assert_eq!(forbidden("").response().status(), StatusCode::FORBIDDEN);
        assert_eq!(
            BoxedAppError::from(DieselError::NotFound)
                .response()
                .status(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(not_found("").response().status(), StatusCode::NOT_FOUND);

        // All other error types are converted to internal server errors
        assert_eq!(
            internal("").response().status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            BoxedAppError::from(serde_json::Error::custom("ExpectedColon"))
                .response()
                .status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            BoxedAppError::from(std::io::Error::new(::std::io::ErrorKind::Other, ""))
                .response()
                .status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
