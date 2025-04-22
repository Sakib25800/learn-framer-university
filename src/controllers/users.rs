use axum::{Extension, Json};
use lfu_database::models::user::UserModel;

use crate::{
    util::errors::{AppErrorResponse, AppResult},
    views::AuthenticatedUser,
};

/// Retrieve a user's profile.
#[utoipa::path(
    get,
    path = "/v1/users/me",
    tag = "users",
    responses(
        (status = 200, body = AuthenticatedUser, description = "successful operation"),
        (status = 400, body = AppErrorResponse, description = "unauthorized")
    )
)]
pub async fn me(Extension(user): Extension<UserModel>) -> AppResult<Json<AuthenticatedUser>> {
    Ok(Json(AuthenticatedUser {
        id: user.id,
        email: user.email,
        email_verified: user.email_verified,
        image: user.image,
        role: user.role,
    }))
}

#[cfg(test)]
mod tests {
    use crate::tests::mocks::{RequestHelper, TestApp};
    use serde_json::json;

    #[sqlx::test]
    async fn user_me_success(pool: sqlx::PgPool) {
        let (_, _, user) = TestApp::init().with_user(pool).await;
        let res = user.get("/v1/users/me").await;
        let user_model = user.as_model();

        res.assert_status_ok();
        res.assert_json(&json!({
            "id": user_model.id.to_string(),
            "email": user_model.email,
            "email_verified": user_model.email_verified,
            "image": user_model.image,
            "role": user_model.role,
        }));
    }

    #[sqlx::test]
    async fn admin_me_success(pool: sqlx::PgPool) {
        let (_, _, _, admin) = TestApp::init().with_admin(pool).await;
        let res = admin.get("/v1/users/me").await;
        let admin_model = admin.as_model();

        res.assert_status_ok();
        res.assert_json(&json!({
            "id": admin_model.id.to_string(),
            "email": admin_model.email,
            "email_verified": admin_model.email_verified,
            "image": admin_model.image,
            "role": admin_model.role,
        }));
    }

    #[sqlx::test]
    async fn anon_me_error(pool: sqlx::PgPool) {
        let (_, anon) = TestApp::init().empty(pool).await;
        let anon = anon.get("/v1/users/me").await;

        anon.assert_status_unauthorized();
        anon.assert_json(&json!({
            "detail": "Invalid or missing authentication",
            "status": 401,
            "title": "Unauthorized"
        }));
    }
}
