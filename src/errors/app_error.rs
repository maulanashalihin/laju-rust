use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

/// Unified application error type.
/// Semua error di app ini dikonversi ke AppError, lalu implement IntoResponse
/// supaya Axum bisa langsung return sebagai HTTP response.
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Internal(String),
    BadRequest(String),
    Database(String),
    Template(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
            AppError::Template(msg) => write!(f, "Template error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into())
            }
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Database(msg) => {
                tracing::error!("Database error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into())
            }
            AppError::Template(msg) => {
                tracing::error!("Template error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Template error".into())
            }
        };

        (status, axum::Json(serde_json::json!({
            "error": message,
        }))).into_response()
    }
}

/// Helper macro/konversi dari error eksternal
impl From<rocksdb::Error> for AppError {
    fn from(err: rocksdb::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<sailfish::RenderError> for AppError {
    fn from(err: sailfish::RenderError) -> Self {
        AppError::Template(err.to_string())
    }
}
