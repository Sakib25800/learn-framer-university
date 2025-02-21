use crate::tests::util::{test_app::TestApp, RequestHelper};
use serde_json::json;

#[tokio::test(flavor = "multi_thread")]
async fn me_should_return_user_data() {
    let (_, _, user) = TestApp::init().with_user().await;
    let user_model = user.as_model();

    let response = user.get("/v1/users/me").await;

    response.assert_status_ok();
    response.assert_json(&json!({
        "id": user_model.id,
        "email": user_model.email,
        "email_verified": user_model.email_verified,
        "image": user_model.image
    }));
}

#[tokio::test(flavor = "multi_thread")]
async fn me_should_reject_unauthenticated_request() {
    let (_, anon) = TestApp::init().empty().await;

    let response = anon.get("/v1/users/me").await;

    response.assert_status_unauthorized();
    response.assert_json(&json!({
        "title": "Unauthorized",
        "detail": "Invalid or missing authentication",
        "status": 401
    }));
}

#[tokio::test(flavor = "multi_thread")]
async fn me_should_work_with_admin_user() {
    let (_, _, admin) = TestApp::init().with_admin().await;
    let admin_model = admin.as_model();

    let response = admin.get("/v1/users/me").await;

    response.assert_status_ok();
    response.assert_json(&json!({
        "id": admin_model.id,
        "email": admin_model.email,
        "email_verified": admin_model.email_verified,
        "image": admin_model.image
    }));
}
