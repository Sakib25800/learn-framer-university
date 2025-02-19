use axum::Json;

use crate::{util::errors::AppResult, views::MessageResponse};

/// Health check
#[utoipa::path(
    get,
    path = "/v1",
    responses(
        (status = 200, body = MessageResponse, description = "successful operation"),
    ),
)]
pub async fn health_check() -> AppResult<Json<MessageResponse>> {
    Ok(Json(MessageResponse {
        message: "Ok".to_string(),
    }))
}
