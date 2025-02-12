use crate::tests::util::{test_app::TestApp, RequestHelper};
use http::StatusCode;
use serde_json::json;

#[tokio::test(flavor = "multi_thread")]
async fn me() {
    let (_, anon, user) = TestApp::init().with_user().await;

    let response = anon.get("/api/v1/me").expect_failure().await;
    response.assert_status(StatusCode::FORBIDDEN);
    response.assert_json(&json!({
        "errors": [
            {
                "detail": "Missing authorization header"
            }
        ]
    }));

    let response = user.get("/api/v1/me").await;
    let user_model = user.as_model();

    response.assert_status(StatusCode::OK);
    response.assert_json(&json!({
        "id": user_model.id,
        "email": user_model.email,
        "email_verified": user_model.email_verified,
        "image": null,
    }));
}
