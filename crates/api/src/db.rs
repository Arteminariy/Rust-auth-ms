//! Database pool + migration runner.
//!
//! Migrations are read from the workspace-root `migrations/` directory via
//! `diesel_migrations::embed_migrations!`. For K8s/init-container/sidecar
//! deployment, set `RUN_MIGRATIONS=false` in the API container and run
//! migrations as a separate Job.

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../../migrations");

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_pool(database_url: &str) -> anyhow::Result<DbPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(16)
        .build(manager)
        .map_err(|e| anyhow::anyhow!("failed to build DB pool: {e}"))?;
    Ok(pool)
}

pub fn run_migrations(pool: &DbPool) -> anyhow::Result<()> {
    let mut conn = pool
        .get()
        .map_err(|e| anyhow::anyhow!("getting connection for migrations: {e}"))?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("running migrations: {e}"))?;
    tracing::info!("migrations applied");
    Ok(())
}
