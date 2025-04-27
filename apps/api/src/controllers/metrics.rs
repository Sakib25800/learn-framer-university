use crate::app::AppState;
use crate::util::errors::{not_found, unauthorized, AppResult};
use axum::extract::Path;
use http::header;
use http::request::Parts;
use prometheus::proto::MetricFamily;
use prometheus::TextEncoder;
use tokio::task::spawn_blocking;

/// Retrieve specific metrics `/api/private/metrics/{kind}`.
pub async fn prometheus(
    state: AppState,
    req: Parts,
    Path(kind): Path<String>,
) -> AppResult<String> {
    if let Some(expected_token) = &state.config.metrics_authorization_token {
        let provided_token = req
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "));

        if provided_token != Some(expected_token.as_str()) {
            return Err(unauthorized(
                "Invalid or missing metrics authorization token",
            ));
        }
    } else {
        // To avoid accidentally leaking metrics if the environment variable is not set, prevent
        // access to any metrics endpoint if the authorization token is not configured.
        return Err(not_found(
            "Metrics are disabled on this framer-university instance",
        ));
    }

    let metrics: Vec<MetricFamily> = match kind.as_str() {
        "service" => state.service_metrics.gather(state.db()).await?,
        "instance" => spawn_blocking(move || state.instance_metrics.gather(&state)).await??,
        _ => return Err(not_found("Metrics type not found")),
    };

    Ok(TextEncoder::new().encode_to_string(&metrics)?)
}
