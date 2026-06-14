//! Small helpers shared by the route modules.

use auth_ms_core::ServiceError;

use crate::error::AppError;

/// Convenience wrapper for service-layer calls: takes a closure that
/// returns `Result<T, ServiceError>` and converts the error to
/// `AppError` on the way out. Saves a `.map_err(AppError::from)` at
/// every call site. The closure runs on the tokio blocking pool.
pub async fn run_service<T, F>(f: F) -> Result<T, AppError>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, ServiceError> + Send + 'static,
{
    tokio::task::spawn_blocking(move || f().map_err(AppError::from))
        .await
        .map_err(|e| AppError::Internal(format!("blocking task failed: {e}")))?
}
