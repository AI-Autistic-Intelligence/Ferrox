use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

/// A standard global error type for the application.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Internal Server Error")]
    InternalServerError(#[from] anyhow::Error), // Requires anyhow, or we can just use generic String/Box<dyn Error>

    #[error("Database Error: {0}")]
    DatabaseError(String),
}

/// Standardized JSON response format for errors
#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::InternalServerError(err) => {
                // In production, we might not want to expose the internal error details
                eprintln!("Internal Server Error: {}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
            AppError::DatabaseError(msg) => {
                eprintln!("Database Error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database Error".to_string(),
                )
            }
        };

        let body = Json(ErrorResponse {
            status: status.as_u16(),
            message,
        });

        (status, body).into_response()
    }
}

pub fn setup() {
    println!("yalc-errors initialized: Provides global AppError and IntoResponse for Axum.");
}
