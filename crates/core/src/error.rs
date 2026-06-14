//! Domain errors that flow from repositories → services → HTTP layer.
//!
//! The HTTP layer maps each variant to an HTTP status code via `AppError::IntoResponse`.

use crate::auth::JwtError;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("forbidden: {0}")]
    Forbidden(String),

    #[error("unprocessable entity: {0}")]
    UnprocessableEntity(String),

    #[error("internal server error: {0}")]
    Internal(String),
}

impl From<diesel::result::Error> for ServiceError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => ServiceError::NotFound("Resource not found".into()),
            other => ServiceError::Internal(other.to_string()),
        }
    }
}

impl From<argon2::password_hash::Error> for ServiceError {
    fn from(e: argon2::password_hash::Error) -> Self {
        ServiceError::Internal(e.to_string())
    }
}

impl From<JwtError> for ServiceError {
    fn from(e: JwtError) -> Self {
        ServiceError::Unauthorized(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ServiceError>;
