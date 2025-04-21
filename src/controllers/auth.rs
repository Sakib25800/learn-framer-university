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

/// Sign in the user and send a sign-in email
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

    match db.users.find_by_email(email).await {
        Ok(user) => {
            db.verification_tokens.delete_all(email).await?;
            user
        }
        Err(_) => db.users.create(email, UserRole::User).await?,
    };

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

/// Verify the user's email
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
        return Err(unauthorized("Verification token has expired"));
    }

    let user = db
        .users
        .find_by_email(&verification_token.identifier)
        .await
        .map_err(|_| unauthorized("Email does not exist"))?;

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

    // Set user email as verified
    db.users.verify_email(user.id).await?;

    // Delete the used verification token
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
    use serde_json::json;

    async fn signin_request(
        pool: sqlx::PgPool,
        body: serde_json::Value,
    ) -> (TestApp, MockAnonymous, TestResponse) {
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
    async fn signin_success_response(pool: sqlx::PgPool) {
        let (_app, _anon, res) = signin_request(
            pool,
            json!({
                "email": "new_user@example.com"
            }),
        )
        .await;

        res.assert_status_ok();
        res.assert_json(&json!({
            "message": "We've sent you an email",
        }));
    }

    #[sqlx::test]
    async fn signin_sends_email(pool: sqlx::PgPool) {
        let (app, _anon, _res) = signin_request(
            pool,
            json!({
                "email": "new_user@example.com"
            }),
        )
        .await;

        assert_snapshot!(app.emails_snapshot().await);

        let emails = app.emails().await;
        let _token = extract_token_from_signin_email(&emails);
    }
}
