//! Core business logic for the auth microservice.
//!
//! Pure layer: models, repositories, services. No HTTP, no I/O frameworks
//! (axum, tower) here — those live in `auth-ms-api`. The split lets us
//! reuse the business logic in other surfaces (CLI, gRPC) later without
//! dragging in the HTTP stack.

#![allow(clippy::result_large_err)] // TODO: tighten as types stabilise
