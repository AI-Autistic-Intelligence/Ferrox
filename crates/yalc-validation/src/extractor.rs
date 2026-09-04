use axum::{
    async_trait,
    extract::{FromRequest, Request, rejection::JsonRejection},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;
use yalc_errors::AppError;

/// ValidatedJson is an Axum extractor that behaves like Zod's `.parse()`.
/// It consumes the request body as JSON, parses it into type T, and immediately
/// calls `.validate()` on it. If validation fails, it throws a formatted 400 Bad Request error.
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // First, extract the JSON normally. 
        // If it's malformed JSON, axum returns a JsonRejection.
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| AppError::ValidationError(format!("Invalid JSON payload: {}", err)))?;

        // Second, run the validator validations.
        // This is the "AutoZod" phase.
        value.validate().map_err(|err| {
            // We can map validation errors nicely. For now, we stringify it.
            AppError::ValidationError(format!("Validation failed: {}", err))
        })?;

        // If everything is correct, wrap it and return
        Ok(ValidatedJson(value))
    }
}
