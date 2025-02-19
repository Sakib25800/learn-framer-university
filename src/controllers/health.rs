use axum::Json;

use crate::{
    util::errors::{AppErrorResponse, AppResult},
    views::MessageResponse,
};

/// Health check
#[utoipa::path(
    get,
    path = "/v1",
    responses(
        (status = OK, body = MessageResponse, description = "successful operation"),
        (status = 400, body = AppErrorResponse, description = "failed operation"),
    ),
)]
pub async fn health_check() -> AppResult<Json<MessageResponse>> {
    Ok(Json(MessageResponse {
        message: "Ok".to_string(),
    }))
}
