use axum::{Extension, Json};
use lfu_database::models::user::UserModel;

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
pub async fn me(Extension(user): Extension<UserModel>) -> AppResult<Json<Me>> {
    let UserModel {
        id,
        email,
        email_verified,
        image,
        ..
    } = user;

    Ok(Json(Me {
        id,
        email,
        email_verified,
        image,
    }))
}
