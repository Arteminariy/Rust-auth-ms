//! HTTP-layer error type. Implements `IntoResponse` so handlers can `?` into it.

use auth_ms_core::ServiceError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("unprocessable entity: {0}")]
    UnprocessableEntity(String),
    #[error("internal: {0}")]
    Internal(String),
}

impl From<ServiceError> for AppError {
    fn from(e: ServiceError) -> Self {
        match e {
            ServiceError::NotFound(m) => AppError::NotFound(m),
            ServiceError::Unauthorized(m) => AppError::Unauthorized(m),
            ServiceError::Forbidden(m) => AppError::Forbidden(m),
            ServiceError::UnprocessableEntity(m) => AppError::UnprocessableEntity(m),
            ServiceError::Internal(m) => AppError::Internal(m),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, m.as_str()),
            AppError::Unauthorized(m) => (StatusCode::UNAUTHORIZED, m.as_str()),
            AppError::Forbidden(m) => (StatusCode::FORBIDDEN, m.as_str()),
            AppError::UnprocessableEntity(m) => (StatusCode::UNPROCESSABLE_ENTITY, m.as_str()),
            AppError::Internal(m) => {
                tracing::error!(error = %m, "internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
        };
        let body = json!({ "code": status.as_u16(), "message": message });
        (status, Json(body)).into_response()
    }
}
