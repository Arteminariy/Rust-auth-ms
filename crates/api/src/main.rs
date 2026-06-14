//! HTTP entry point for the auth microservice.

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::Router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod bootstrap;
mod config;
mod db;
mod error;
mod extractors;
mod routes;
mod state;
mod util;

use crate::config::AppConfig;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    // .env in dev (no-op in prod where env is set by the orchestrator)
    let _ = dotenvy::dotenv();

    // tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = AppConfig::from_env().context("loading AppConfig from env")?;
    tracing::info!(bind = %cfg.bind_addr, "starting auth-ms-api");

    // DB pool + migrations
    let pool = db::establish_pool(&cfg.database_url).context("establishing DB pool")?;
    if cfg.run_migrations {
        db::run_migrations(&pool).context("running diesel migrations")?;
    }

    // Bootstrap: admin role + admin user (idempotent)
    bootstrap::init_database(&pool, cfg.initial_admin_password.as_deref());

    // Build services and state
    let bind_addr = cfg.bind_addr;
    let state = Arc::new(AppState::new(pool, cfg));

    // Build router
    let app = Router::new()
        .merge(routes::auth::router())
        .merge(routes::users::router())
        .merge(routes::roles::router())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .with_context(|| format!("binding to {bind_addr}"))?;
    let local: SocketAddr = listener.local_addr()?;
    tracing::info!(addr = %local, "listening");
    axum::serve(listener, app).await?;
    Ok(())
}
