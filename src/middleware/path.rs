use crate::util::errors::{bad_request, internal};
use crate::util::errors::{AppResult, BoxedAppError};
use axum::{
    extract::{rejection::PathRejection, FromRequestParts},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

pub struct ValidatedPath<T>(pub T);

impl<S, T> FromRequestParts<S> for ValidatedPath<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = BoxedAppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> AppResult<Self> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => match rejection {
                PathRejection::FailedToDeserializePathParams(inner) => {
                    let kind = inner.into_kind();
                    Err(bad_request(kind.to_string()))
                }
                _ => Err(internal("Unknown path rejection".to_string())),
            },
        }
    }
}
