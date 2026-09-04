use axum::{
    extract::State,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::{PasetoAuth, AuthPayload};

/// A middleware that extracts the PASETO token from the `Authorization: Bearer <token>` header,
/// validates it, and injects the `AuthPayload` into the request extensions.
pub async fn require_auth<B>(
    State(auth_engine): State<Arc<PasetoAuth>>,
    mut req: Request<B>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .filter(|value| value.starts_with("Bearer "))
        .map(|value| &value[7..]);

    let token = match auth_header {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    match auth_engine.validate_token(token) {
        Ok(payload) => {
            // 1. Inject payload into local request context (for Monolith controllers)
            req.extensions_mut().insert(payload.clone());
            
            // 2. Inject claims as HTTP Headers (API Gateway Pattern for Downstream Microservices)
            // This prevents downstream microservices from having to re-validate the cryptographic signature
            // or query the database, achieving zero-trust security without performance penalties.
            if let Ok(user_id_val) = header::HeaderValue::from_str(&payload.user_id) {
                req.headers_mut().insert("x-ferrox-user-id", user_id_val);
            }
            if let Ok(role_val) = header::HeaderValue::from_str(&payload.role) {
                req.headers_mut().insert("x-ferrox-user-role", role_val);
            }

            Ok(next.run(req).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

/// An extractor guard for RBAC (Role-Based Access Control).
/// Use it to protect routes, e.g., `.route_layer(axum::middleware::from_fn(|req, next| require_role(req, next, "admin")))`
pub async fn require_role<B>(
    req: Request<B>,
    next: Next,
    required_role: &str,
) -> Result<Response, StatusCode> {
    // Attempt to extract the AuthPayload previously injected by `require_auth`
    let auth_payload = req.extensions().get::<AuthPayload>();

    match auth_payload {
        Some(payload) if payload.role == required_role => {
            Ok(next.run(req).await)
        }
        _ => Err(StatusCode::FORBIDDEN), // User exists but lacks the required role
    }
}
