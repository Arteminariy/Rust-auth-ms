//! /roles/* — create, get one, get list, update, delete.
//!
//! Create / update / delete are admin-only. Read endpoints require any
//! authenticated user.

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use uuid::Uuid;

use auth_ms_core::models::roles::{NewRole, Role, UpdateRole};
use auth_ms_core::pagination::{List, ResponsePagination};

use crate::error::AppError;
use crate::extractors::{AdminUser, AuthUser};
use crate::state::AppState;
use crate::util::run_service;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/roles",
            post(create_role).get(get_list),
        )
        .route(
            "/roles/{role_id}",
            get(get_role).put(update_role).delete(delete_role),
        )
}

#[derive(Debug, Deserialize)]
struct PaginationQuery {
    size: Option<i64>,
    page: Option<i64>,
}

async fn create_role(
    State(state): State<Arc<AppState>>,
    _admin: AdminUser,
    Json(dto): Json<NewRole>,
) -> Result<Json<Role>, AppError> {
    let roles = state.roles.clone();
    let res = run_service(move || roles.create(dto)).await?;
    Ok(Json(res))
}

async fn get_role(
    State(state): State<Arc<AppState>>,
    _user: AuthUser,
    Path(role_id): Path<Uuid>,
) -> Result<Json<Role>, AppError> {
    let roles = state.roles.clone();
    let res = run_service(move || roles.get_one(role_id)).await?;
    Ok(Json(res))
}

async fn get_list(
    State(state): State<Arc<AppState>>,
    _user: AuthUser,
    Query(q): Query<PaginationQuery>,
) -> Result<Json<List<Role>>, AppError> {
    let pagination = ResponsePagination {
        size: q.size.unwrap_or(10),
        page: q.page.unwrap_or(1),
    };
    let roles = state.roles.clone();
    let res = run_service(move || roles.get_list(pagination)).await?;
    Ok(Json(res))
}

async fn update_role(
    State(state): State<Arc<AppState>>,
    _admin: AdminUser,
    Path(role_id): Path<Uuid>,
    Json(dto): Json<UpdateRole>,
) -> Result<Json<Role>, AppError> {
    let roles = state.roles.clone();
    let res = run_service(move || roles.update(role_id, dto)).await?;
    Ok(Json(res))
}

async fn delete_role(
    State(state): State<Arc<AppState>>,
    _admin: AdminUser,
    Path(role_id): Path<Uuid>,
) -> Result<(), AppError> {
    let roles = state.roles.clone();
    run_service(move || roles.delete(role_id)).await?;
    Ok(())
}
