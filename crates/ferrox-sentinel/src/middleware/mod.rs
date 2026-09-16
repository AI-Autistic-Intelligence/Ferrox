use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use tower::{Layer, Service};
use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;

use crate::SentinelEngine;
use ferrox_security::threats::SecurityManager;

pub mod polymorphic_routes;

/// Axum Layer for Ferrox Sentinel AI/ML Security Analytics
#[derive(Clone)]
pub struct SentinelLayer {
    engine: Arc<SentinelEngine>,
    security_manager: Arc<SecurityManager>,
}

impl SentinelLayer {
    pub fn new(engine: Arc<SentinelEngine>, security_manager: Arc<SecurityManager>) -> Self {
        Self {
            engine,
            security_manager,
        }
    }
}

impl<S> Layer<S> for SentinelLayer {
    type Service = SentinelMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SentinelMiddleware {
            inner,
            engine: self.engine.clone(),
            security_manager: self.security_manager.clone(),
        }
    }
}

/// Axum Middleware Service generic over Body type B
#[derive(Clone)]
pub struct SentinelMiddleware<S> {
    inner: S,
    engine: Arc<SentinelEngine>,
    security_manager: Arc<SecurityManager>,
}

impl<S, B> Service<Request<B>> for SentinelMiddleware<S>
where
    S: Service<Request<B>, Response = Response> + Send + Sync + 'static + Clone,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let mut inner = self.inner.clone();
        let engine = self.engine.clone();
        let security_manager = self.security_manager.clone();

        Box::pin(async move {
            let client_ip = req
                .headers()
                .get("x-forwarded-for")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("127.0.0.1")
                .to_string();

            // 1. Check if IP is already banned
            if security_manager.is_ip_banned(&client_ip).await {
                let err_body = serde_json::json!({
                    "error": "IP Banned",
                    "message": "Access blocked due to previous high-severity security violations."
                });
                return Ok((StatusCode::FORBIDDEN, axum::Json(err_body)).into_response());
            }

            // 2. Evaluate request threat score using Sentinel ML Engine
            let uri_str = req.uri().to_string();
            let headers_str = format!("{:?}", req.headers());

            let assessment = engine.evaluate_request(&client_ip, &uri_str, &headers_str, "");

            // 3. Handle high-risk threat scores
            if assessment.threat_score > 0.85 {
                let _ = security_manager.record_request(&client_ip).await;

                let response_body = serde_json::json!({
                    "error": "Threat Blocked",
                    "threat_score": assessment.threat_score,
                    "rationale": assessment.rationale
                });
                return Ok((StatusCode::BAD_REQUEST, axum::Json(response_body)).into_response());
            }

            // 4. Pass request to inner service and inject OWASP Security Headers
            let mut response = inner.call(req).await?;
            let headers = response.headers_mut();

            if !headers.contains_key("strict-transport-security") {
                headers.insert("strict-transport-security", "max-age=31536000; includeSubDomains; preload".parse().unwrap());
            }
            if !headers.contains_key("content-security-policy") {
                headers.insert("content-security-policy", "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; frame-ancestors 'none';".parse().unwrap());
            }
            if !headers.contains_key("x-frame-options") {
                headers.insert("x-frame-options", "DENY".parse().unwrap());
            }
            if !headers.contains_key("x-content-type-options") {
                headers.insert("x-content-type-options", "nosniff".parse().unwrap());
            }
            if !headers.contains_key("x-xss-protection") {
                headers.insert("x-xss-protection", "1; mode=block".parse().unwrap());
            }
            if !headers.contains_key("referrer-policy") {
                headers.insert("referrer-policy", "strict-origin-when-cross-origin".parse().unwrap());
            }

            Ok(response)
        })
    }
}
