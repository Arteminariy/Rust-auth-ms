//! Axum extractor: any valid Bearer token. Yields the access claim.

use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use uuid::Uuid;

use auth_ms_core::auth::decode_access;
use auth_ms_core::AccessClaim;

use crate::error::AppError;
use crate::state::AppState;

/// The authenticated user, populated from a valid `Authorization: Bearer <jwt>` header.
pub struct AuthUser {
    pub user_id: Uuid,
    pub claim: AccessClaim,
}

#[axum::async_trait]
impl FromRequestParts<std::sync::Arc<AppState>> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &std::sync::Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?;
        let token = header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Authorization must be a Bearer token".into()))?
            .trim();
        let claim = decode_access(token, &state.cfg.jwt_secret)
            .map_err(|e| AppError::Unauthorized(e.to_string()))?;
        Ok(Self {
            user_id: claim.id,
            claim,
        })
    }
}

