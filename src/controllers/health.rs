use axum::Json;

use crate::{
    util::errors::{AppErrorResponse, AppResult},
    views::MessageResponse,
};

/// Health check
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

// async fn signin_should_succeed_with_valid_email() {
//     let (app, anon) = TestApp::init().empty().await;

//     let sign_in_response = anon
//         .post("/v1/auth/signin")
//         .json(&json!({
//             "email": "new_user@example.com"
//         }))
//         .await;

//     sign_in_response.assert_status(StatusCode::OK);
//     sign_in_response.assert_json(&json!({
//         "message": "We've sent you an email",
//     }));

//     let emails = app.emails().await;

//     assert_snapshot!(app.emails_snapshot().await);

//     // Retrieve the continue token
//     let continue_token = extract_token_from_signin_email(&emails);

//     // Continue with token
//     let continue_path = format!("/v1/auth/continue/{continue_token}");
//     let continue_response = anon.get(&continue_path).await;

//     continue_response.assert_status_ok();

//     let json_continue_response = continue_response.json::<serde_json::Value>();

//     match json_continue_response {
//         serde_json::Value::Object(map) => {
//             assert!(map.contains_key("access_token"));
//             assert!(map.contains_key("refresh_token"));
//         }
//         _ => panic!("Expected JSON object"),
//     }
// }
