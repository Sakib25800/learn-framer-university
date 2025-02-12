use axum::Json;

use crate::{util::errors::AppResult, views::SuccessResponse};

/// Health check
#[utoipa::path(
    get,
    path = "/api/v1",
    responses((status = 200, description = "Successful Response")),
)]
pub async fn health_check() -> AppResult<Json<SuccessResponse<()>>> {
    Ok(Json(SuccessResponse {
        message: "Ok".to_string(),
        data: None,
    }))
}
