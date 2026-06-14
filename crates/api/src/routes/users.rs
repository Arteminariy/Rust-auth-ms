//! /users/* — get one, get list, update, delete.

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use uuid::Uuid;

use auth_ms_core::models::users::{UpdateUserDto, UserDto};
use auth_ms_core::pagination::{List, ResponsePagination};

use crate::error::AppError;
use crate::extractors::AuthUser;
use crate::state::AppState;
use crate::util::run_service;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(get_list))
        .route("/users/{user_id}", get(get_user).put(update_user).delete(delete_user))
}

#[derive(Debug, Deserialize)]
struct PaginationQuery {
    size: Option<i64>,
    page: Option<i64>,
}

async fn get_user(
    State(state): State<Arc<AppState>>,
    _user: AuthUser,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserDto>, AppError> {
    let users = state.users.clone();
    let res = run_service(move || users.get_one(user_id)).await?;
    Ok(Json(res))
}

async fn get_list(
    State(state): State<Arc<AppState>>,
    _user: AuthUser,
    Query(q): Query<PaginationQuery>,
) -> Result<Json<List<UserDto>>, AppError> {
    let pagination = ResponsePagination {
        size: q.size.unwrap_or(10),
        page: q.page.unwrap_or(1),
    };
    let users = state.users.clone();
    let res = run_service(move || users.get_list(pagination)).await?;
    Ok(Json(res))
}

async fn update_user(
    State(state): State<Arc<AppState>>,
    _user: AuthUser,
    Path(user_id): Path<Uuid>,
    Json(dto): Json<UpdateUserDto>,
) -> Result<Json<UserDto>, AppError> {
    let users = state.users.clone();
    let res = run_service(move || users.update(user_id, dto)).await?;
    Ok(Json(res))
}

async fn delete_user(
    State(state): State<Arc<AppState>>,
    _user: AuthUser,
    Path(user_id): Path<Uuid>,
) -> Result<(), AppError> {
    let users = state.users.clone();
    run_service(move || users.delete(user_id)).await?;
    Ok(())
}
