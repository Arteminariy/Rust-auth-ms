//! The shared application state. Held in an `Arc<AppState>` and passed to
//! every handler via `axum::extract::State`.

use std::sync::Arc;

use auth_ms_email::SmtpSender;
use auth_ms_core::repositories::{RolesRepository, UserRepository};
use auth_ms_core::services::{AuthService, RolesService, UserService};

use crate::config::AppConfig;
use crate::db::DbPool;

#[allow(dead_code)] // `pool` and `email` are wired for Phase 2; ignore unused here
pub struct AppState {
    pub pool: DbPool,
    pub cfg: AppConfig,
    pub email: Arc<SmtpSender>,
    pub auth: AuthService,
    pub users: UserService,
    pub roles: RolesService,
}

impl AppState {
    pub fn new(pool: DbPool, cfg: AppConfig) -> Self {
        let user_repo = UserRepository { pool: pool.clone() };
        let roles_repo = RolesRepository { pool: pool.clone() };
        let auth = AuthService {
            user_repository: UserRepository { pool: pool.clone() },
            jwt_secret: cfg.jwt_secret.clone(),
            access_ttl_secs: cfg.access_ttl_secs,
            refresh_ttl_secs: cfg.refresh_ttl_secs,
        };
        let users = UserService { repo: user_repo };
        let roles = RolesService { repo: roles_repo };
        let email = Arc::new(SmtpSender::new("noreply@localhost"));
        Self { pool, cfg, email, auth, users, roles }
    }
}
