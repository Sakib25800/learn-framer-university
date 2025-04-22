use axum::Json;
use chrono::Utc;
use lfu_database::models::user::UserRole;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::{
    app::AppState,
    auth::generate_access_token,
    config::Server,
    middleware::{json::JsonBody, path::ValidatedPath},
    util::errors::{unauthorized, AppErrorResponse, AppResult},
    views::{MessageResponse, VerifiedEmailResponse},
};

#[derive(Deserialize, Validate, ToSchema)]
pub struct AuthSignInBody {
    #[validate(email)]
    email: String,
}

#[derive(Deserialize, Validate)]
pub struct VerifyEmailQueryParams {
    #[validate(length(min = 1))]
    token: String,
}

/// Sign in the user and send a sign-in email.
#[utoipa::path(
    post,
    path = "/v1/auth/signin",
    tag = "auth",
    request_body = AuthSignInBody,
    responses(
        (status = 200, body = MessageResponse, description = "successful operation"),
        (status = 400, body = AppErrorResponse, description = "failed operation"),
    ),
)]
pub async fn signin(
    state: AppState,
    JsonBody(body): JsonBody<AuthSignInBody>,
) -> AppResult<Json<MessageResponse>> {
    let db = state.db();
    let email = &body.email;

    if let Err(_) = db.users.find_by_email(email).await {
        db.verification_tokens.delete_all(email).await?;
    }

    let verification_token = db
        .verification_tokens
        .create(
            email.to_owned(),
            state.config.email_verification_expiration_hours,
        )
        .await?;

    let signin_email = AuthSignInEmail {
        app_url: &state.config.app_url,
        token: &verification_token.token,
    };

    state.emails.send(&body.email, signin_email).await?;

    Ok(Json(MessageResponse {
        message: "We've sent you an email".to_owned(),
    }))
}

#[derive(Deserialize, Validate)]
pub struct AuthSignInParams {
    #[validate(length(min = 1))]
    pub token: String,
}

/// Verify the user's email and create a new user.
#[utoipa::path(
    get,
    path = "/v1/auth/continue/{token}",
    params(
        ("token" = String, Path, description = "Token used to verify email")
    ),
    responses(
        (status = 200, body = VerifiedEmailResponse, description = "successful operation"),
        (status = 400, body = AppErrorResponse, description = "failed operation"),
    ),
    tag = "auth",
)]
pub async fn continue_signin(
    state: AppState,
    ValidatedPath(params): ValidatedPath<AuthSignInParams>,
) -> AppResult<Json<VerifiedEmailResponse>> {
    let token = params.token;
    let db = state.db();

    let verification_token = db
        .verification_tokens
        .find_by_token(&token)
        .await
        .map_err(|_| unauthorized("Invalid verification token"))?;

    if verification_token.expires < Utc::now() {
        return Err(unauthorized("Expired verification token"));
    }

    let user = match db.users.find_by_email(&verification_token.identifier).await {
        Ok(user) => user,
        Err(_) => {
            db.users
                .create(&verification_token.identifier, UserRole::User)
                .await?
        }
    };

    let Server {
        jwt_secret,
        jwt_access_token_expiration_hours,
        jwt_refresh_token_expiration_days,
        ..
    } = state.config.as_ref();
    let access_token = generate_access_token(
        jwt_secret,
        jwt_access_token_expiration_hours,
        user.id,
        user.email,
    )?;
    let refresh_token = db
        .refresh_tokens
        .create(user.id, *jwt_refresh_token_expiration_days)
        .await?;

    // Set user email as verified.
    db.users.verify_email(user.id).await?;

    // Delete the used verification token.
    db.verification_tokens
        .delete(&verification_token.identifier, token.as_str())
        .await?;

    Ok(Json(VerifiedEmailResponse {
        access_token,
        refresh_token: refresh_token.token,
    }))
}

pub struct AuthSignInEmail<'a> {
    pub app_url: &'a str,
    pub token: &'a str,
}

impl crate::email::Email for AuthSignInEmail<'_> {
    fn subject(&self) -> String {
        "Activation link for Framer University".into()
    }

    fn body(&self) -> String {
        format!(
            "Hey there! Welcome to Framer University.\nPlease click the link below to sign in: {app_url}/api/continue/{token}",
            app_url = self.app_url,
            token = self.token,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::mocks::{MockAnonymous, RequestHelper, TestApp};
    use axum_test::TestResponse;
    use insta::assert_snapshot;
    use serde_json::{json, Value};
    use sqlx::PgPool;

    async fn signin_request(pool: PgPool, body: Value) -> (TestApp, MockAnonymous, TestResponse) {
        let (app, anon) = TestApp::init().empty(pool).await;
        let res = anon.post("/v1/auth/signin").json(&body).await;
        (app, anon, res)
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

    #[sqlx::test]
    async fn signin_valid_email_success(pool: PgPool) {
        let (_, _, res) = signin_request(
            pool,
            json!({
                "email": "unverified@example.com"
            }),
        )
        .await;

        res.assert_status_ok();
        res.assert_json(&json!({
            "message": "We've sent you an email",
        }));
    }

    #[sqlx::test]
    async fn signin_existing_user_success(pool: PgPool) {
        let (_, _, user) = TestApp::init().with_user(pool).await;

        let res = user
            .post("/v1/auth/signin")
            .json(&json!({
                "email": "verified@example.com"
            }))
            .await;

        res.assert_status_ok();
    }

    #[sqlx::test]
    async fn signin_deletes_existing_tokens(pool: PgPool) {
        let email = "user@example.com";

        let (app, anon) = TestApp::init().empty(pool).await;

        anon.post("/v1/auth/signin")
            .json(&json!({
                "email": email
            }))
            .await;
        anon.post("/v1/auth/signin")
            .json(&json!({
                "email": email
            }))
            .await;

        app.db()
            .verification_tokens
            .count()
            .await
            .unwrap()
            .map(|count| assert_eq!(count, 1));
    }

    #[sqlx::test]
    async fn signin_sends_email(pool: PgPool) {
        let (app, _, _) = signin_request(
            pool,
            json!({
                "email": "foo@example.com"
            }),
        )
        .await;

        let emails = app.emails().await;
        assert_eq!(emails.len(), 1);
        assert_snapshot!(app.emails_snapshot().await);
    }

    #[sqlx::test]
    async fn signin_creates_verification_token(pool: PgPool) {
        let (app, _, _) = signin_request(
            pool,
            json!({
                "email": "foo@example.com"
            }),
        )
        .await;

        app.db()
            .verification_tokens
            .count()
            .await
            .unwrap()
            .map(|count| assert_eq!(count, 1));
    }

    #[sqlx::test]
    async fn signin_invalid_email_error(pool: PgPool) {
        let (_, _, res) = signin_request(
            pool,
            json!({
                "email": "invalidemail"
            }),
        )
        .await;

        res.assert_status_bad_request();
        res.assert_json(&json!({
            "title": "Invalid request",
            "detail": "Invalid email address",
            "status": 400
        }));
    }

    #[sqlx::test]
    async fn signin_missing_email_error(pool: PgPool) {
        let (_, _, res) = signin_request(pool, json!({})).await;

        res.assert_status_bad_request();
        res.assert_json(&json!({
            "title": "Invalid request",
            "detail": "Invalid JSON",
            "status": 400
        }));
    }

    #[sqlx::test]
    async fn continue_signin_success(pool: PgPool) {
        let (app, anon, _) = signin_request(
            pool,
            json!({
                "email": "unverified@example.com"
            }),
        )
        .await;

        let emails = app.emails().await;
        let token = extract_token_from_signin_email(&emails);

        let res = anon.get(&format!("/v1/auth/continue/{token}")).await;

        res.assert_status_ok();
        match res.json::<serde_json::Value>() {
            Value::Object(map) => {
                assert!(map.contains_key("access_token"));
                assert!(map.contains_key("refresh_token"));
            }
            _ => panic!("Expected JSON object"),
        }
    }

    #[sqlx::test]
    async fn continue_signin_creats_user(pool: PgPool) {
        let email = "unverified@example.com";
        let (app, anon, _) = signin_request(
            pool,
            json!({
                "email": email
            }),
        )
        .await;

        let emails = app.emails().await;
        let token = extract_token_from_signin_email(&emails);

        anon.get(&format!("/v1/auth/continue/{token}")).await;

        let user = app.db().users.find_by_email(email).await.unwrap();
        assert!(user.email_verified.is_some());
    }

    #[sqlx::test]
    async fn continue_signin_invalid_error(pool: PgPool) {
        let (_, anon) = TestApp::init().empty(pool).await;

        let res = anon.get("/v1/auth/continue/invalid_token").await;

        res.assert_status_unauthorized();
        res.assert_json(&json!({
            "title": "Unauthorized",
            "detail": "Invalid verification token",
            "status": 401
        }));
    }

    #[sqlx::test]
    async fn continue_signin_expired_error(pool: PgPool) {
        let email = "foo@example.com";
        let (app, anon, _) = signin_request(pool, json!({ "email": email })).await;

        let emails = app.emails().await;
        let token = extract_token_from_signin_email(&emails);

        app.db()
            .verification_tokens
            .expire_by_identifier(email)
            .await
            .unwrap();

        let res = anon.get(&format!("/v1/auth/continue/{token}")).await;

        res.assert_status_unauthorized();
        res.assert_json(&json!({
            "title": "Unauthorized",
            "detail": "Expired verification token",
            "status": 401
        }));
    }

    #[sqlx::test]
    async fn continue_signin_used_token_error(pool: PgPool) {
        let (app, anon, _res) = signin_request(
            pool,
            json!({
                "email": "foo@example.com"
            }),
        )
        .await;

        let emails = app.emails().await;
        let token = extract_token_from_signin_email(&emails);

        let res = anon.get(&format!("/v1/auth/continue/{}", token)).await;
        res.assert_status_ok();

        let res = anon.get(&format!("/v1/auth/continue/{}", token)).await;

        res.assert_status_unauthorized();
        res.assert_json(&json!({
            "title": "Unauthorized",
            "status": 401,
            "detail": "Invalid verification token"
        }));
    }
}
