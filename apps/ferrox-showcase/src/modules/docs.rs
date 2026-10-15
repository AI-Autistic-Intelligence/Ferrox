use axum::{
    response::{Html, Redirect},
    routing::get,
    Json, Router,
};
use serde_json::{json, Value};

pub fn openapi_spec() -> Value {
    json!({
        "openapi": "3.0.3",
        "info": {
            "title": "Ferrox Showcase Application API",
            "version": "0.1.0",
            "description": "Interactive showcase application demonstrating core Ferrox framework features:\n- PASETO v4 token security\n- Multi-transport Axum HTTP adapter\n- Kubernetes health probes (/health/live, /health/ready)\n- Zero-Trust CORS isolation",
            "contact": {
                "name": "Ferrox Framework Team",
                "url": "https://ferrox.dev"
            }
        },
        "servers": [
            {
                "url": "http://localhost:3000",
                "description": "Local Showcase Server"
            }
        ],
        "tags": [
            { "name": "Health", "description": "Liveness & Readiness Probes" },
            { "name": "Auth", "description": "PASETO Token Operations" }
        ],
        "paths": {
            "/health/live": {
                "get": {
                    "tags": ["Health"],
                    "summary": "Kubernetes Liveness Probe",
                    "responses": {
                        "200": { "description": "Process is operational" }
                    }
                }
            },
            "/health/ready": {
                "get": {
                    "tags": ["Health"],
                    "summary": "Kubernetes Readiness Probe",
                    "responses": {
                        "200": { "description": "Service is ready to receive traffic" }
                    }
                }
            },
            "/api/v1/auth/token": {
                "get": {
                    "tags": ["Auth"],
                    "summary": "Generate Demonstration PASETO Token",
                    "responses": {
                        "200": { "description": "Token generated successfully" }
                    }
                }
            }
        }
    })
}

pub async fn swagger_ui() -> Html<String> {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Ferrox Showcase - Swagger UI</title>
  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5.11.0/swagger-ui.css" />
  <style>body { margin: 0; background: #fafafa; } .topbar { display: none; }</style>
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
        presets: [SwaggerUIBundle.presets.apis, SwaggerUIStandalonePreset],
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
