use axum::{Extension, Json};
use lfu_database::models::user::User;

use crate::{
    util::errors::{AppErrorResponse, AppResult},
    views::Me,
};

#[utoipa::path(
    get,
    path = "/v1/users/me",
    tag = "users",
    responses(
        (status = 200, body = Me, description = "successful operation"),
        (status = 400, body = AppErrorResponse, description = "unauthorized")
    )
)]
pub async fn me(Extension(user): Extension<User>) -> AppResult<Json<Me>> {
    let profile = Me {
        id: user.id,
        email: user.email,
        email_verified: user.email_verified,
        image: user.image,
    };

    Ok(Json(profile))
}
