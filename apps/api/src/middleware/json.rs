use crate::util::errors::{bad_request, AppResult, BoxedAppError};
use axum::extract::{FromRequest, Json, Request};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors, ValidationErrorsKind};

fn generate_user_friendly_message(
    field: &str,
    code: &str,
    params: &std::collections::HashMap<std::borrow::Cow<'static, str>, serde_json::Value>,
) -> String {
    match code {
        "email" => "Invalid email address".to_string(),
        "url" => format!("'{}' is not a valid URL", field),
        "length" => {
            let min = params.get("min").and_then(|v| v.as_u64());
            let max = params.get("max").and_then(|v| v.as_u64());
            let equal = params.get("equal").and_then(|v| v.as_u64());

            match (min, max, equal) {
                (Some(min), Some(max), _) => {
                    format!("'{}' must be between {} and {} characters", field, min, max)
                }
                (Some(min), None, _) => format!("'{}' must be at least {} characters", field, min),
                (None, Some(max), _) => format!("'{}' must not exceed {} characters", field, max),
                (_, _, Some(equal)) => format!("'{}' must be exactly {} characters", field, equal),
                _ => format!("'{}' has an invalid length", field),
            }
        }
        "range" => {
            let min = params.get("min").and_then(|v| v.as_f64());
            let max = params.get("max").and_then(|v| v.as_f64());

            match (min, max) {
                (Some(min), Some(max)) => {
                    format!("'{}' must be between {} and {}", field, min, max)
                }
                (Some(min), None) => format!("'{}' must be at least {}", field, min),
                (None, Some(max)) => format!("'{}' must not exceed {}", field, max),
                _ => format!("'{}' is out of the allowed range", field),
            }
        }
        "must_match" => {
            let other = params
                .get("other")
                .and_then(|v| v.as_str())
                .unwrap_or("other field");
            format!("'{}' must match '{}'", field, other)
        }
        "contains" => format!("'{}' must contain the required value", field),
        "does_not_contain" => format!("'{}' contains a forbidden value", field),
        "regex" => format!("'{}' has an invalid format", field),
        "credit_card" => format!("'{}' is not a valid credit card number", field),
        "required" => format!("'{}' is required", field),
        "non_control_character" => format!("'{}' contains invalid control characters", field),
        _ => format!("'{}' failed validation ({})", field, code),
    }
}

fn get_first_validation_error(errors: ValidationErrors) -> String {
    // Find the first field error.
    for (field, kind) in errors.into_errors() {
        match kind {
            ValidationErrorsKind::Field(errs) => {
                if let Some(err) = errs.into_iter().next() {
                    // Use custom message if available, otherwise generate one.
                    if let Some(msg) = err.message {
                        return msg.to_string();
                    } else {
                        return generate_user_friendly_message(&field, &err.code, &err.params);
                    }
                }
            }
            ValidationErrorsKind::Struct(nested_errors) => {
                // Recursively check nested struct errors.
                return get_first_validation_error(*nested_errors);
            }
            ValidationErrorsKind::List(list_errors) => {
                // Get the first error from the list.
                if let Some((index, nested_errors)) = list_errors.into_iter().next() {
                    let nested_message = get_first_validation_error(*nested_errors);
                    return format!("{} at index {}: {}", field, index, nested_message);
                }
            }
        }
    }

    // Fallback if no specific error is found.
    "Invalid data".to_string()
}

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
        data.validate()
            .map_err(|errs| bad_request(get_first_validation_error(errs)))?;
        Ok(JsonBody(data))
    }
}
