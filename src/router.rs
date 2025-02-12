use crate::controllers::*;
use crate::util::errors::not_found;
use crate::{app::AppState, openapi::BaseOpenApi};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{middleware, Json, Router};
use http::{Method, StatusCode};
use utoipa_axum::routes;

pub fn build_axum_router(state: AppState) -> Router<()> {
    let (public_router, public_openapi) = BaseOpenApi::router()
        .routes(routes!(health::health_check))
        .routes(routes!(auth::login, auth::verify))
        .split_for_parts();

    let (protected_router, protected_openapi) = BaseOpenApi::router()
        .routes(routes!(user::me::get_authenticated_user))
        .split_for_parts();

    let protected_router = protected_router.layer(middleware::from_fn_with_state(
        state.clone(),
        crate::middleware::auth::auth,
    ));

    let openapi = public_openapi.merge_from(protected_openapi);

    Router::new()
        .merge(public_router)
        .merge(protected_router)
        .route("/api/private/metrics/{kind}", get(metrics::prometheus))
        .route("/api/openapi.json", get(|| async { Json(openapi) }))
        .fallback(|method: Method| async move {
            match method {
                Method::HEAD => StatusCode::NOT_FOUND.into_response(),
                _ => not_found("Route not found").into_response(),
            }
        })
        .with_state(state)
}
