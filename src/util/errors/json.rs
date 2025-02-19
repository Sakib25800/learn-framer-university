use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::Serialize;
use serde_json::json;
use std::{borrow::Cow, fmt};

use super::{AppError, BoxedAppError};

#[derive(Serialize)]
pub struct ErrorResponse {
    title: String,
    status: u16,
    detail: String,
}

/// Generates a response following [RFC 8707 format](https://datatracker.ietf.org/doc/html/rfc7807)
pub fn json_error(title: &str, status: StatusCode, detail: &str) -> Response {
    let json = json!(ErrorResponse {
        title: title.into(),
        status: status.as_u16(),
        detail: detail.into()
    });
    // Wrap the json value with Json
    (status, Json(json)).into_response()
}

// The following structs wrap owned data and provide a custom message to the user

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
