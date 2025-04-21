//! This module implements several error types and traits.
//!
//! * `util::errors::AppResult` - Failures can be converted into user facing JSON responses. This error type
//!   is more dynamic is box allocated. Low-level errors are typically not converted to user facing errors and most usage
//!   lies within models, controllers, and middleware layers.

use axum::{response::IntoResponse, Extension};
use http::StatusCode;
use std::{
    any::{Any, TypeId},
    borrow::Cow,
    error::Error,
    fmt,
};
use tokio::task::JoinError;
use tracing::*;
use validator::ValidationErrors;

use crate::{email::EmailError, middleware::log_request::ErrorField};
pub use json::{custom, AppErrorResponse};

mod json;

pub type BoxedAppError = Box<dyn AppError>;

pub fn bad_request(detail: impl Into<Cow<'static, str>>) -> BoxedAppError {
    custom("Invalid request", StatusCode::BAD_REQUEST, detail)
}

pub fn forbidden(detail: impl Into<Cow<'static, str>>) -> BoxedAppError {
    custom("Forbidden", StatusCode::FORBIDDEN, detail)
}

pub fn unauthorized(detail: impl Into<Cow<'static, str>>) -> BoxedAppError {
    custom("Unauthorized", StatusCode::UNAUTHORIZED, detail)
}

pub fn not_found(detail: impl Into<Cow<'static, str>>) -> BoxedAppError {
    custom("Not found", StatusCode::NOT_FOUND, detail)
}

/// Returns an error with status 503 and the provided description as JSON
pub fn service_unavailable() -> BoxedAppError {
    custom(
        "Service unavailable",
        StatusCode::SERVICE_UNAVAILABLE,
        "Service unavailable",
    )
}

// =============================================================================
// AppError trait

pub trait AppError: Send + fmt::Display + fmt::Debug + 'static {
    /// Generate an HTTP response for the error
    ///
    /// If none is returned, the error will bubble up the middleware stack
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

impl From<sqlx::Error> for BoxedAppError {
    fn from(err: sqlx::Error) -> BoxedAppError {
        match err {
            sqlx::Error::PoolTimedOut => service_unavailable(),
            sqlx::Error::PoolClosed => service_unavailable(),
            sqlx::Error::WorkerCrashed => service_unavailable(),
            sqlx::Error::RowNotFound => not_found("Resource not found"),
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                bad_request("Resource already exists")
            }
            _ => {
                error!("Database error: {err:?}");
                internal("An internal server error occurred")
            }
        }
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

impl From<EmailError> for BoxedAppError {
    fn from(error: EmailError) -> Self {
        match error {
            EmailError::Address(error) => Box::new(error),
            EmailError::MessageBuilder(error) => Box::new(error),
            EmailError::Transport(error) => {
                error!(?error, "Failed to send email");
                internal("Failed to send the email")
            }
        }
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
