//! Core business logic for the auth microservice.
//!
//! Pure layer: models, repositories, services. No HTTP, no I/O frameworks
//! (axum, tower) here — those live in `auth-ms-api`. The split lets us
//! reuse the business logic in other surfaces (CLI, gRPC) later without
//! dragging in the HTTP stack.

pub mod auth;
pub mod dto;
pub mod error;
pub mod models;
pub mod pagination;
pub mod repositories;
pub mod schema;
pub mod services;

pub use error::{Result, ServiceError};
pub use auth::{AccessClaim, JwtError, RefreshClaim};
