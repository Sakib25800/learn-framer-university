use crate::app::AppState;
use crate::config::Env;
use ::sentry::integrations::tower as sentry_tower;
use axum::middleware::{from_fn, from_fn_with_state};
use axum::Router;
use axum_extra::either::Either;
use axum_extra::middleware::option_layer;
use std::time::Duration;
use tower::layer::util::Identity;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::{CompressionLayer, CompressionLevel};
use tower_http::timeout::{RequestBodyTimeoutLayer, TimeoutLayer};

pub mod app;
pub mod auth;
mod debug;
pub mod json;
pub mod log_request;
pub mod path;
pub mod query;
mod real_ip;
mod update_metrics;

pub fn apply_axum_middleware(state: AppState, router: Router<()>) -> Router {
    let config = &state.config;

    let middlewares = tower::ServiceBuilder::new()
        .layer(sentry_tower::NewSentryLayer::new_from_top())
        .layer(sentry_tower::SentryHttpLayer::with_transaction())
        .layer(from_fn(self::real_ip::middleware))
        .layer(from_fn(log_request::log_requests))
        .layer(CatchPanicLayer::new())
        .layer(from_fn_with_state(
            state.clone(),
            update_metrics::update_metrics,
        ))
        .layer(AddExtensionLayer::new(state.clone()))
        // Optionally print debug information for each request
        // To enable, set the environment variable: `RUST_LOG=crates_io::middleware=debug`
        .layer(conditional_layer(config.env == Env::Development, || {
            from_fn(debug::debug_requests)
        }));

    router
        .layer(middlewares)
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(RequestBodyTimeoutLayer::new(Duration::from_secs(30)))
        .layer(CompressionLayer::new().quality(CompressionLevel::Fastest))
}

pub fn conditional_layer<L, F: FnOnce() -> L>(condition: bool, layer: F) -> Either<L, Identity> {
    option_layer(condition.then(layer))
}
