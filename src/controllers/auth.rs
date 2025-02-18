use axum::Json;
use chrono::Utc;
use utoipa::ToSchema;
use validator::Validate;

use crate::{
    app::AppState,
    auth::generate_tokens,
    middleware::{json::JsonBody, path::ValidatedPath},
    models::{
        user::{NewUser, User},
        verification_token::{NewVerificationToken, VerificationToken},
    },
    util::errors::{auth, AppResult},
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
    responses((status = OK, body = MessageResponse))
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
        Err(_) => NewUser::new(&body.email).create(&mut conn).await?,
    };

    let verification_token = NewVerificationToken::new(
        email.to_owned(),
        state.config.email_verification_expiration_hours,
    )
    .create(&mut conn)
    .await?;

    let signin_email = AuthSignInEmail {
        domain: &state.config.domain_name,
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
        ("email_token" = String, Path, description = "Token used to verify email")
    ),
    responses((status = OK, body = VerifiedEmailResponse)),
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
        Err(_) => return Err(auth("Invalid verification token")),
    };

    if verification_token.expires < Utc::now() {
        return Err(auth("Verification token has expired"));
    }

    let user = match User::find_by_email(&verification_token.identifier, &mut conn).await {
        Ok(user) => user,
        Err(_) => return Err(auth("Email does not exist")),
    };

    let tokens = generate_tokens(
        &state.config.jwt_secret,
        state.config.jwt_access_token_expiration_hours,
        state.config.jwt_refresh_token_expiration_days,
        &user,
    )?;

    // Set user email as verified
    User::verify_email(user.id, &mut conn).await?;

    // Delete the used verification token
    VerificationToken::delete(&verification_token.identifier, token.as_str(), &mut conn).await?;

    Ok(Json(VerifiedEmailResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
    }))
}

pub struct AuthSignInEmail<'a> {
    pub domain: &'a str,
    pub token: &'a str,
}

impl crate::email::Email for AuthSignInEmail<'_> {
    fn subject(&self) -> String {
        "Framer University: Please confirm your email address".into()
    }

    fn body(&self) -> String {
        format!(
            "Hey there! Welcome to Framer University.\nPlease click the link below to sign in: http://{domain}/auth/continue/{token}",
            domain = self.domain,
            token = self.token,
        )
    }
}
