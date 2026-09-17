//! # Mandatory Compliance Enforcer (`unbypassable_enforcer.rs`)
//!
//! Provides a zero-developer-bypass runtime security header enforcer middleware for Axum/Ferrox applications.
//! Automatically strips technology disclosure headers and forcibly applies OWASP-compliant security headers to all outbound responses.

use axum::{
    http::{HeaderName, HeaderValue, Request, Response},
    middleware::Next,
};

/// List of dangerous tech disclosure headers to strip from responses
pub const DANGEROUS_LEAK_HEADERS: &[&str] = &[
    "server",
    "x-powered-by",
    "x-runtime",
    "x-aspnet-version",
    "x-debug-token",
    "x-generator",
    "x-served-by",
    "x-laravel-version",
    "x-express-version",
];

/// Mandatory OWASP security headers to force on all responses
pub const MANDATORY_SECURITY_HEADERS: &[(&str, &str)] = &[
    ("strict-transport-security", "max-age=63072000; includeSubDomains; preload"),
    ("x-frame-options", "DENY"),
    ("x-content-type-options", "nosniff"),
    ("referrer-policy", "strict-origin-when-cross-origin"),
    ("permissions-policy", "geolocation=(), microphone=(), camera=()"),
    ("content-security-policy", "default-src 'self'; frame-ancestors 'none';"),
    ("x-ferrox-shield", "enforced-v1.0"),
];

/// Struct representing the Unbypassable Security Enforcer engine
#[derive(Debug, Clone, Default)]
pub struct MandatoryComplianceEnforcer;

impl MandatoryComplianceEnforcer {
    /// Sanitizes and fortifies an HTTP response header map.
    ///
    /// Strips any technology leak headers and forces OWASP security headers.
    pub fn enforce_headers<B>(mut response: Response<B>) -> Response<B> {
        let headers = response.headers_mut();

        // 1. Strip all technology disclosure headers regardless of what handlers set
        for leak_header in DANGEROUS_LEAK_HEADERS {
            if let Ok(name) = HeaderName::from_bytes(leak_header.as_bytes()) {
                headers.remove(name);
            }
        }

        // 2. Set an anonymized, uniform server identity
        headers.insert(
            HeaderName::from_static("server"),
            HeaderValue::from_static("Ferrox-Shield/1.0"),
        );

        // 3. Force-inject all mandatory security headers
        for (name, val) in MANDATORY_SECURITY_HEADERS {
            if let (Ok(h_name), Ok(h_val)) = (
                HeaderName::from_bytes(name.as_bytes()),
                HeaderValue::from_str(val),
            ) {
                headers.insert(h_name, h_val);
            }
        }

        response
    }
}

/// Axum middleware function to apply unbypassable security enforcement
pub async fn mandatory_compliance_middleware(
    req: Request<axum::body::Body>,
    next: Next,
) -> axum::response::Response {
    let response = next.run(req).await;
    MandatoryComplianceEnforcer::enforce_headers(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{HeaderValue, Response};

    #[test]
    fn test_unbypassable_enforcer_strips_leak_headers() {
        let mut builder = Response::builder();
        builder = builder.header("x-powered-by", "PHP/8.2.0");
        builder = builder.header("server", "Apache/2.4.41 (Ubuntu)");
        builder = builder.header("x-runtime", "12ms");

        let response = builder.body(()).unwrap();
        let enforced = MandatoryComplianceEnforcer::enforce_headers(response);

        assert!(enforced.headers().get("x-powered-by").is_none());
        assert!(enforced.headers().get("x-runtime").is_none());
        assert_eq!(
            enforced.headers().get("server").unwrap(),
            &HeaderValue::from_static("Ferrox-Shield/1.0")
        );
    }

    #[test]
    fn test_unbypassable_enforcer_injects_security_headers() {
        let response = Response::builder().body(()).unwrap();
        let enforced = MandatoryComplianceEnforcer::enforce_headers(response);

        assert_eq!(
            enforced.headers().get("x-frame-options").unwrap(),
            &HeaderValue::from_static("DENY")
        );
        assert_eq!(
            enforced.headers().get("x-content-type-options").unwrap(),
            &HeaderValue::from_static("nosniff")
        );
        assert!(enforced.headers().get("strict-transport-security").is_some());
        assert!(enforced.headers().get("content-security-policy").is_some());
        assert_eq!(
            enforced.headers().get("x-ferrox-shield").unwrap(),
            &HeaderValue::from_static("enforced-v1.0")
        );
    }

    #[test]
    fn test_developer_override_prevention() {
        // Simulate a developer handler explicitly trying to set a dangerous header or weak X-Frame-Options
        let mut builder = Response::builder();
        builder = builder.header("x-frame-options", "ALLOWALL");
        builder = builder.header("x-express-version", "4.18.2");

        let response = builder.body(()).unwrap();
        let enforced = MandatoryComplianceEnforcer::enforce_headers(response);

        // Weak header must be overwritten to DENY
        assert_eq!(
            enforced.headers().get("x-frame-options").unwrap(),
            &HeaderValue::from_static("DENY")
        );
        // Leak header must be stripped
        assert!(enforced.headers().get("x-express-version").is_none());
    }
}
