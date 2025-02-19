use axum::Json;
use chrono::Utc;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use lfu_database::models::{
    refresh_token::NewRefreshToken,
    user::{NewUser, User},
    verification_token::{NewVerificationToken, VerificationToken},
};

use crate::{
    app::AppState,
    auth::generate_access_token,
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
    let mut conn = state.database.get().await?;
    let email = &body.email;

    match User::find_by_email(email, &mut conn).await {
        Ok(user) => {
            // Revoke all previously generated verification tokens
            VerificationToken::delete_all(&user.email, &mut conn).await?;
            user
        }
        Err(_) => NewUser::new(&body.email, false).insert(&mut conn).await?,
    };

    let verification_token = NewVerificationToken::new(
        email.to_owned(),
        state.config.email_verification_expiration_hours,
    )
    .create(&mut conn)
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
    let mut conn = state.database.get().await?;

    let verification_token = match VerificationToken::find_by_token(&token, &mut conn).await {
        Ok(token) => token,
        Err(_) => return Err(unauthorized("Invalid verification token")),
    };

    if verification_token.expires < Utc::now() {
        return Err(unauthorized("Verification token has expired"));
    }

    let user = match User::find_by_email(&verification_token.identifier, &mut conn).await {
        Ok(user) => user,
        Err(_) => return Err(unauthorized("Email does not exist")),
    };

    let crate::config::Server {
        jwt_secret,
        jwt_access_token_expiration_hours,
        jwt_refresh_token_expiration_days,
        ..
    } = state.config.as_ref();
    let access_token = generate_access_token(jwt_secret, jwt_access_token_expiration_hours, &user)?;
    let refresh_token = NewRefreshToken::new(user.id, *jwt_refresh_token_expiration_days)
        .insert(&mut conn)
        .await?;

    // Set user email as verified
    User::verify_email(user.id, &mut conn).await?;

    // Delete the used verification token
    VerificationToken::delete(&verification_token.identifier, token.as_str(), &mut conn).await?;

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
        "Framer University: Please confirm your email address".into()
    }

    fn body(&self) -> String {
        format!(
            "Hey there! Welcome to Framer University.\nPlease click the link below to sign in: {app_url}/api/continue/{token}",
            app_url = self.app_url,
            token = self.token,
        )
    }
}
