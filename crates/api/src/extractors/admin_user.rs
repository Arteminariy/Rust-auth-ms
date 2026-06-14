//! Axum extractor: any valid Bearer token *with* an admin role.

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use std::sync::Arc;

use crate::error::AppError;
use crate::extractors::auth_user::AuthUser;
use crate::state::AppState;

/// Admin user, populated from a valid `Authorization: Bearer *** header
/// that also has a `role_id` claim. The inner `AuthUser` is exposed for
/// downstream handlers that want to read `user_id` or `name`.
pub struct AdminUser(#[allow(dead_code)] pub AuthUser);

#[axum::async_trait]
impl FromRequestParts<Arc<AppState>> for AdminUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let user = AuthUser::from_request_parts(parts, state).await?;
        if user.claim.role_id.is_none() {
            return Err(AppError::Forbidden("Admin role required".into()));
        }
        Ok(Self(user))
    }
}
