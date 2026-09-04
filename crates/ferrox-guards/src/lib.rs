use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode, header},
};
use ferrox_security::paseto::PasetoAuth;
use ferrox_errors::AppError;

pub struct RequireRole(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for RequireRole
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = parts.headers.get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized("Invalid token format".into()));
        }

        let token = &auth_header[7..];

        // In a real implementation, you would pass the secret via State or env.
        // For boilerplate, we'll assume a dummy secret or validation logic.
        // let auth = PasetoAuth::new(Secret::new("...".into())).unwrap();
        // let claims = auth.validate_token(token)?;
        
        // Let's simulate role extraction from claims
        let role = "admin"; // Simulated

        // The exact required role is usually checked via a parameter in Axum.
        // Since Rust traits don't support const generics for strings yet easily in extractors,
        // we extract the role and the controller checks it.
        // Or we use this Extractor just to get the Role string.
        
        Ok(RequireRole(role.to_string()))
    }
}
