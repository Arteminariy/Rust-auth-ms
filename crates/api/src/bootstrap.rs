//! Bootstrap: idempotently create the default `admin` role and `admin/admin123`
//! user on first start. Mirrors the original Rocket `init_database` exactly.

use diesel::prelude::*;
use uuid::Uuid;

use auth_ms_core::auth::hash_password;
use auth_ms_core::models::roles::NewRole;
use auth_ms_core::models::users::CreateUserEntity;
use auth_ms_core::schema::{roles, users};
use crate::db::DbPool;

pub fn init_database(pool: &DbPool, initial_password: Option<&str>) {
    let mut conn = match pool.get() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(error = %e, "bootstrap: failed to get DB connection");
            return;
        }
    };

    let roles_count: i64 = roles::table
        .count()
        .get_result(&mut conn)
        .unwrap_or(0);

    if roles_count == 0 {
        let new_role = NewRole { name: "admin".to_string() };
        if let Err(e) = diesel::insert_into(roles::table)
            .values(&new_role)
            .execute(&mut conn)
        {
            tracing::error!(error = %e, "bootstrap: failed to insert admin role");
            return;
        }
        tracing::info!("bootstrap: created 'admin' role");
    }

    let users_count: i64 = users::table.count().get_result(&mut conn).unwrap_or(0);
    if users_count == 0 {
        let admin_role_id: Uuid = match roles::table
            .filter(roles::name.eq("admin"))
            .select(roles::id)
            .first(&mut conn)
        {
            Ok(id) => id,
            Err(e) => {
                tracing::error!(error = %e, "bootstrap: admin role not found after insert");
                return;
            }
        };

        let password = initial_password.unwrap_or("admin123");
        let hashed = match hash_password(password) {
            Ok(h) => h,
            Err(e) => {
                tracing::error!(error = %e, "bootstrap: failed to hash admin password");
                return;
            }
        };

        let new_user = CreateUserEntity {
            name: "admin".to_string(),
            role_id: Some(admin_role_id),
            password_hash: hashed,
        };
        if let Err(e) = diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut conn)
        {
            tracing::error!(error = %e, "bootstrap: failed to insert admin user");
            return;
        }
        tracing::info!(initial_password = %initial_password.is_some(), "bootstrap: created 'admin' user");
    }
}
