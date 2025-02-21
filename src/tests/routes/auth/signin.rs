use crate::tests::util::{test_app::TestApp, RequestHelper};
use http::StatusCode;
use insta::assert_snapshot;
use serde_json::json;

#[tokio::test(flavor = "multi_thread")]
async fn signin_should_succeed_with_valid_email() {
    let (app, anon) = TestApp::init().empty().await;

    let sign_in_response = anon
        .post("/v1/auth/signin")
        .json(&json!({
            "email": "new_user@example.com"
        }))
        .await;

    sign_in_response.assert_status(StatusCode::OK);
    sign_in_response.assert_json(&json!({
        "message": "We've sent you an email",
    }));

    let emails = app.emails().await;

    assert_snapshot!(app.emails_snapshot().await);

    // Retrieve the continue token
    let continue_token = extract_token_from_signin_email(&emails);

    // Continue with token
    let continue_path = format!("/v1/auth/continue/{continue_token}");
    let continue_response = anon.get(&continue_path).await;

    continue_response.assert_status_ok();

    let json_continue_response = continue_response.json::<serde_json::Value>();

    match json_continue_response {
        serde_json::Value::Object(map) => {
            assert!(map.contains_key("access_token"));
            assert!(map.contains_key("refresh_token"));
        }
        _ => panic!("Expected JSON object"),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn signin_should_reject_with_invalid_token() {
    let (_, anon) = TestApp::init().empty().await;

    let continue_path = "/v1/auth/continue/invalid_token";
    let response = anon.get(continue_path).await;

    response.assert_status_unauthorized();
    response.assert_json(&json!({
        "title": "Unauthorized",
        "detail": "Invalid verification token",
        "status": 401
    }));
}

#[tokio::test(flavor = "multi_thread")]
async fn signin_should_reject_invalid_email_format() {
    let (app, anon) = TestApp::init().empty().await;

    let response = anon
        .post("/v1/auth/signin")
        .json(&json!({
            "email": "not-an-email"
        }))
        .await;

    response.assert_status_bad_request();

    assert!(app.emails().await.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn signin_should_reject_when_email_missing() {
    let (app, anon) = TestApp::init().empty().await;

    let response = anon.post("/v1/auth/signin").json(&json!({})).await;

    response.assert_status_bad_request();

    assert!(app.emails().await.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn signin_should_work_for_existing_user() {
    let (app, anon, user) = TestApp::init().with_user().await;

    let response = anon
        .post("/v1/auth/signin")
        .json(&json!({
            "email": user.as_model().email
        }))
        .await;

    response.assert_status_ok();
    response.assert_json(&json!({
        "message": "We've sent you an email"
    }));

    let emails = app.emails().await;

    assert_eq!(emails.len(), 1);
}

fn extract_token_from_signin_email(emails: &[String]) -> String {
    let body = emails
        .iter()
        .find(|m| m.contains("Subject: Activation link for Framer University"))
        .expect("Missing email");

    let after_prefix = body
        .split("/continue/")
        .nth(1)
        .expect("Couldn't find token start");

    let token = after_prefix
        .split_whitespace()
        .next()
        .expect("Couldn't find token end");

    token.to_string()
}
