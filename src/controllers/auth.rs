use axum::Json;
use chrono::Utc;
use utoipa::ToSchema;
use validator::Validate;

use crate::{
    app::AppState,
    auth::generate_tokens,
    middleware::{json::JsonBody, query::Query},
    models::{
        user::{NewUser, User},
        verification_token::{NewVerificationToken, VerificationToken},
    },
    util::errors::{auth, AppResult},
    views::{SuccessResponse, VerifiedEmailResponse},
};

#[derive(Deserialize, Validate, ToSchema)]
pub struct AuthLoginBody {
    #[validate(email)]
    email: String,
    #[validate(length(min = 2))]
    name: String,
}

#[derive(Deserialize, Validate)]
pub struct VerifyEmailQueryParams {
    #[validate(length(min = 1))]
    token: String,
}

/// Login the user
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "auth",
    request_body = AuthLoginBody,
    responses((status = 200, description = "Successful Response")),
)]
pub async fn login(
    state: AppState,
    JsonBody(body): JsonBody<AuthLoginBody>,
) -> AppResult<Json<SuccessResponse<()>>> {
    let mut conn = state.database.get().await?;
    let email = &body.email;

    match User::find_by_email(email, &mut conn).await {
        Ok(user) => user,
        Err(_) => {
            NewUser::new(&body.name, &body.email)
                .create(&mut conn)
                .await?
        }
    };

    let verification_token = NewVerificationToken::new(
        email.to_owned(),
        state.config.email_verification_expiration_hours,
    )
    .create(&mut conn)
    .await?;

    Ok(Json(SuccessResponse {
        message: format!("We've sent you an email: {}", verification_token.token),
        data: None,
    }))
}

/// Verify the user's email
#[utoipa::path(
    get,
    path = "/api/v1/auth/verify",
    params(
        ("token" = String, Path, description = "Email verification token")
    ),
    tag = "auth"
)]
pub async fn verify(
    state: AppState,
    Query(query): Query<VerifyEmailQueryParams>,
) -> AppResult<Json<VerifiedEmailResponse>> {
    let token = &query.token;
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
    VerificationToken::delete(&verification_token.identifier, &token, &mut conn).await?;

    Ok(Json(VerifiedEmailResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
    }))
}
