//! /auth/* — login, register, refresh, change-password.

use std::sync::Arc;

use axum::extract::State;
use axum::routing::{post, put};
use axum::{Json, Router};

use auth_ms_core::dto::change_password::ChangePasswordDto;
use auth_ms_core::dto::login::LoginData;
use auth_ms_core::dto::refresh::RefreshData;
use auth_ms_core::dto::token::TokenResponse;
use auth_ms_core::models::users::CreateUserDto;

use crate::error::AppError;
use crate::extractors::AuthUser;
use crate::state::AppState;
use crate::util::run_service;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .route("/auth/refresh", post(refresh))
        .route("/auth/change-password", put(change_password))
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<LoginData>,
) -> Result<Json<TokenResponse>, AppError> {
    let auth = state.auth.clone();
    let res = run_service(move || auth.login(dto)).await?;
    Ok(Json(res))
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<CreateUserDto>,
) -> Result<Json<TokenResponse>, AppError> {
    let auth = state.auth.clone();
    let res = run_service(move || auth.register(dto)).await?;
    Ok(Json(res))
}

async fn refresh(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<RefreshData>,
) -> Result<Json<TokenResponse>, AppError> {
    let auth = state.auth.clone();
    let res = run_service(move || auth.refresh(dto)).await?;
    Ok(Json(res))
}

async fn change_password(
    State(state): State<Arc<AppState>>,
    user: AuthUser,
    Json(dto): Json<ChangePasswordDto>,
) -> Result<(), AppError> {
    let auth = state.auth.clone();
    run_service(move || auth.change_password(user.user_id, dto)).await?;
    Ok(())
}
