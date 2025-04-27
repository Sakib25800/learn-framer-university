use crate::util::errors::{bad_request, AppResult, BoxedAppError};
use axum::extract::FromRequestParts;
use http::request::Parts;
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct Query<T>(pub T);

impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = BoxedAppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> AppResult<Self> {
        let query = parts.uri.query().unwrap_or_default();
        let query: T = serde_urlencoded::from_str(query).map_err(|e| bad_request(e.to_string()))?;
        query.validate()?;
        Ok(Query(query))
    }
}
