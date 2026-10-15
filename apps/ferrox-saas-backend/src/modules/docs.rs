use axum::{
    response::{Html, Redirect},
    routing::get,
    Json, Router,
};
use serde_json::{json, Value};

/// Returns the OpenAPI 3.0.3 specification for the Ferrox SaaS Backend
pub fn openapi_spec() -> Value {
    json!({
        "openapi": "3.0.3",
        "info": {
            "title": "Ferrox Enterprise SaaS Backend API",
            "version": "0.8.0",
            "description": "Production-grade reference SaaS backend implemented in Rust using Axum and Ferrox Core Framework.\nFeatures PASETO v4 token authentication, RBAC, tenant isolation, and distributed observability.",
            "contact": {
                "name": "Ferrox Engineering Team",
                "url": "https://ferrox.dev"
            }
        },
        "servers": [
            {
                "url": "http://localhost:8080",
                "description": "Local Development Server"
            }
        ],
        "tags": [
            { "name": "Health", "description": "Liveness and Readiness Probes" },
            { "name": "Auth", "description": "PASETO v4 Authentication, TOTP 2FA, and Token Refresh" },
            { "name": "Users", "description": "User profile management and multi-tenancy" },
            { "name": "Admin", "description": "Administrative dashboards, security audits, and tenant quotas" },
            { "name": "Mailer", "description": "Transactional and template-based emails" },
            { "name": "Notifications", "description": "Push and in-app notifications" }
        ],
        "paths": {
            "/health/live": {
                "get": {
                    "tags": ["Health"],
                    "summary": "Liveness Probe",
                    "description": "Checks if the service process is active.",
                    "responses": {
                        "200": { "description": "Service is alive" }
                    }
                }
            },
            "/health/ready": {
                "get": {
                    "tags": ["Health"],
                    "summary": "Readiness Probe",
                    "description": "Checks connectivity to PostgreSQL, Redis, and internal security systems.",
                    "responses": {
                        "200": { "description": "Service is ready to handle traffic" }
                    }
                }
            },
            "/api/v1/auth/register": {
                "post": {
                    "tags": ["Auth"],
                    "summary": "Register New User",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["email", "password"],
                                    "properties": {
                                        "email": { "type": "string", "format": "email" },
                                        "password": { "type": "string", "minLength": 8 }
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "201": { "description": "User registered successfully" },
                        "400": { "description": "Validation error" }
                    }
                }
            },
            "/api/v1/auth/login": {
                "post": {
                    "tags": ["Auth"],
                    "summary": "Authenticate User with PASETO v4",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "required": ["email", "password"],
                                    "properties": {
                                        "email": { "type": "string" },
                                        "password": { "type": "string" }
                                    }
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": { "description": "Authentication successful, returns PASETO v4 token" },
                        "401": { "description": "Invalid credentials" }
                    }
                }
            },
            "/api/v1/users/me": {
                "get": {
                    "tags": ["Users"],
                    "summary": "Get Current Authenticated User Profile",
                    "security": [{ "PasetoAuth": [] }],
                    "responses": {
                        "200": { "description": "User profile retrieved" },
                        "401": { "description": "Missing or invalid token" }
                    }
                }
            },
            "/api/v1/admin/dashboard": {
                "get": {
                    "tags": ["Admin"],
                    "summary": "Administrative System Overview",
                    "security": [{ "PasetoAuth": [] }],
                    "responses": {
                        "200": { "description": "Admin metrics and tenant stats" },
                        "403": { "description": "Forbidden: Requires ADMIN role" }
                    }
                }
            }
        },
        "components": {
            "securitySchemes": {
                "PasetoAuth": {
                    "type": "http",
                    "scheme": "bearer",
                    "bearerFormat": "PASETO-v4",
                    "description": "Enter your PASETO v4.local token prefixed with Bearer"
                }
            }
        }
    })
}

/// Serves standalone interactive Swagger UI
pub async fn swagger_ui() -> Html<String> {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Ferrox SaaS Backend - Swagger UI</title>
  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5.11.0/swagger-ui.css" />
  <link rel="icon" type="image/png" href="https://raw.githubusercontent.com/rust-lang/rust-artwork/master/logo/rust-logo-blk.svg" />
  <style>
    body { margin: 0; background: #fafafa; }
    .topbar { display: none; }
  </style>
</head>
<body>
  <div id="swagger-ui"></div>
  <script src="https://unpkg.com/swagger-ui-dist@5.11.0/swagger-ui-bundle.js"></script>
  <script src="https://unpkg.com/swagger-ui-dist@5.11.0/swagger-ui-standalone-preset.js"></script>
  <script>
    window.onload = function() {
      SwaggerUIBundle({
        url: "/docs/openapi.json",
        dom_id: '#swagger-ui',
        deepLinking: true,
        presets: [
          SwaggerUIBundle.presets.apis,
          SwaggerUIStandalonePreset
        ],
        layout: "BaseLayout"
      });
    };
  </script>
</body>
</html>"#;

    Html(html.to_string())
}

pub fn router() -> Router {
    Router::new()
        .route("/docs", get(swagger_ui))
        .route("/swagger-ui", get(|| async { Redirect::permanent("/docs") }))
        .route("/docs/openapi.json", get(|| async { Json(openapi_spec()) }))
}
