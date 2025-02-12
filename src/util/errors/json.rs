use axum::response::{IntoResponse, Response};
use axum_extra::json;
use http::StatusCode;
use std::{borrow::Cow, fmt};

use super::{AppError, BoxedAppError};

/// Generates a response with the provided status and description as JSON
pub fn json_error(detail: &str, status: StatusCode) -> Response {
    let json = json!({"errors": [{"detail": detail}]});
    (status, json).into_response()
}

/// The following structs wrap owned data and provide a custom message to the user

pub fn custom(status: StatusCode, detail: impl Into<Cow<'static, str>>) -> BoxedAppError {
    Box::new(CustomApiError {
        status,
        detail: detail.into(),
    })
}

#[derive(Debug, Clone)]
pub struct CustomApiError {
    pub status: StatusCode,
    pub detail: Cow<'static, str>,
}

impl fmt::Display for CustomApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.detail.fmt(f)
    }
}

impl AppError for CustomApiError {
    fn response(&self) -> Response {
        json_error(&self.detail, self.status)
    }
}
