//! Environment-loaded configuration. Loaded once at startup.

use std::env;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub bind_addr: SocketAddr,
    pub database_url: String,
    pub jwt_secret: String,
    pub access_ttl_secs: u64,
    pub refresh_ttl_secs: u64,
    pub run_migrations: bool,
    pub initial_admin_password: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let bind_addr: SocketAddr = env::var("BIND_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:8000".into())
            .parse()?;

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?;

        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| anyhow::anyhow!("JWT_SECRET must be set"))?;
        if jwt_secret.len() < 32 {
            anyhow::bail!("JWT_SECRET must be at least 32 chars; got {}", jwt_secret.len());
        }

        let access_ttl_secs = env::var("JWT_ACCESS_TTL")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(900); // 15 min
        let refresh_ttl_secs = env::var("JWT_REFRESH_TTL")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(2_592_000); // 30 days

        let run_migrations = env::var("RUN_MIGRATIONS")
            .map(|v| !matches!(v.as_str(), "0" | "false" | "no" | ""))
            .unwrap_or(true);

        let initial_admin_password = env::var("INITIAL_ADMIN_PASSWORD").ok();

        Ok(Self {
            bind_addr,
            database_url,
            jwt_secret,
            access_ttl_secs,
            refresh_ttl_secs,
            run_migrations,
            initial_admin_password,
        })
    }
}
