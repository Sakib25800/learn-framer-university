use crate::util::errors::{bad_request, AppResult, BoxedAppError};
use axum::extract::{FromRequest, Json, Request};
use serde::de::DeserializeOwned;
use validator::Validate;

pub struct JsonBody<T>(pub T);

impl<T, S> FromRequest<S> for JsonBody<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = BoxedAppError;

    async fn from_request(req: Request, state: &S) -> AppResult<Self> {
        let Json(data) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| bad_request("Invalid JSON"))?;
        data.validate().map_err(|_| bad_request("Invalid JSON"))?;
        Ok(JsonBody(data))
    }
}
