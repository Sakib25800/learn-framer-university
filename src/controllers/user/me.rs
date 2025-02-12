use crate::{models::user::User, util::errors::AppResult, views::Me};
use axum::{Extension, Json};

/// Get the currently authenticated user
#[utoipa::path(
    get,
    path = "/api/v1/me",
    tag = "users",
    security(("bearer" = [])),
    responses((status = 200, description = "Successful Response")),
)]
pub async fn get_authenticated_user(Extension(user): Extension<User>) -> AppResult<Json<Me>> {
    Ok(Json(Me {
        id: user.id,
        email: user.email,
        email_verified: user.email_verified,
        image: user.image,
    }))
}
