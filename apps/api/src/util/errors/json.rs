use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::Serialize;
use serde_json::json;
use std::{borrow::Cow, fmt};

use super::{AppError, BoxedAppError};

#[derive(Serialize, utoipa::ToSchema)]
pub struct AppErrorResponse {
    /// A short, human-readable summary of the error. It should
    /// not change from occurrence to occurrence of the error.
    #[schema(example = "Unauthorized")]
    title: String,

    /// The HTTP status code.
    #[schema(example = "400")]
    status: u16,

    /// A human-readable explanation specific to
    /// this occurrence of the error.
    #[schema(example = "The verification link has expired. Please request a new one.")]
    detail: String,
}

/// Generates a response following [RFC 8707 format](https://datatracker.ietf.org/doc/html/rfc7807).
pub fn json_error(title: &str, status: StatusCode, detail: &str) -> Response {
    let json = json!(AppErrorResponse {
        title: title.into(),
        status: status.as_u16(),
        detail: detail.into()
    });
    // Wrap the json value with Json.
    (status, Json(json)).into_response()
}

// The following structs wrap owned data and provide a custom message to the user.

pub fn custom(
    title: impl Into<Cow<'static, str>>,
    status: StatusCode,
    detail: impl Into<Cow<'static, str>>,
) -> BoxedAppError {
    Box::new(CustomApiError {
        status,
        title: title.into(),
        detail: detail.into(),
    })
}

#[derive(Debug, Clone)]
pub struct CustomApiError {
    pub status: StatusCode,
    pub title: Cow<'static, str>,
    pub detail: Cow<'static, str>,
}

impl fmt::Display for CustomApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.detail.fmt(f)
    }
}

impl AppError for CustomApiError {
    fn response(&self) -> Response {
        json_error(&self.title, self.status, &self.detail)
    }
}
