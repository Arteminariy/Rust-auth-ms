//! HTTP entry point for the auth microservice.
//!
//! Phase 0a: workspace scaffolding. The full Axum port of the original Rocket
//! endpoints is delivered in subsequent commits on the `phase-0-axum-rewrite`
//! branch. This binary boots a minimal server with two routes (`/` and
//! `/livez`) wired through tower's `TraceLayer` so we can verify the
//! workspace + dependency graph is wired correctly end-to-end.

use anyhow::{Context, Result};
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env in dev (no-op if absent)
    let _ = dotenvy::dotenv();

    // Init tracing from RUST_LOG (default: info)
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/livez", get(livez))
        .layer(TraceLayer::new_for_http());

    let bind = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8000".into());
    let addr: std::net::SocketAddr = bind.parse().context("BIND_ADDR is not a valid socket address")?;
    tracing::info!(%addr, "auth-ms-api listening");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn root() -> &'static str {
    "rust-auth-ms: phase-0-axum-rewrite scaffolding"
}

async fn livez() -> &'static str {
    "ok"
}
