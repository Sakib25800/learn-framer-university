use axum::Json;

use crate::{
    util::errors::{AppErrorResponse, AppResult},
    views::MessageResponse,
};

/// Health check.
#[utoipa::path(
    get,
    path = "/",
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

#[cfg(test)]
mod tests {
    use crate::tests::mocks::{RequestHelper, TestApp};
    use serde_json::json;

    #[sqlx::test]
    async fn health_check(pool: sqlx::PgPool) {
        let (_, anon) = TestApp::init().empty(pool).await;
        anon.get("/").await.assert_json(&json!({
            "message": "Ok"
        }));
    }
}
