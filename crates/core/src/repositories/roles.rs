use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use crate::models::roles::{NewRole, Role, UpdateRole};
use crate::pagination::{List, RequestPagination, ResponsePagination};
use crate::schema::roles;

#[derive(Clone)]
pub struct RolesRepository {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl RolesRepository {
    pub fn create(&self, role_dto: NewRole) -> Result<Role, diesel::result::Error> {
        let mut conn = self.pool.get().expect("diesel pool: failed to get connection");
        diesel::insert_into(roles::table)
            .values(&role_dto)
            .get_result::<Role>(&mut conn)
    }

    pub fn get_one(&self, role_id: Uuid) -> Result<Role, diesel::result::Error> {
        let mut conn = self.pool.get().expect("diesel pool: failed to get connection");
        roles::table.find(role_id).get_result::<Role>(&mut conn)
    }

    pub fn get_list(
        &self,
        pagination: ResponsePagination,
    ) -> Result<List<Role>, diesel::result::Error> {
        let mut conn = self.pool.get().expect("diesel pool: failed to get connection");

        let total_count: i64 = roles::table.count().get_result(&mut conn)?;

        let items: Vec<Role> = roles::table
            .limit(pagination.size)
            .offset((pagination.page.max(1) - 1) * pagination.size)
            .load::<Role>(&mut conn)?;

        let total_pages = if pagination.size == 0 {
            0
        } else {
            (total_count as f64 / pagination.size as f64).ceil() as i64
        };

        Ok(List {
            pagination: RequestPagination { total_count, total_pages },
            items,
        })
    }

    pub fn update(
        &self,
        role_id: Uuid,
        role_dto: UpdateRole,
    ) -> Result<Role, diesel::result::Error> {
        let mut conn = self.pool.get().expect("diesel pool: failed to get connection");
        diesel::update(roles::table.filter(roles::id.eq(role_id)))
            .set(&role_dto)
            .get_result::<Role>(&mut conn)
    }

    pub fn delete(&self, role_id: Uuid) -> Result<(), diesel::result::Error> {
        let mut conn = self.pool.get().expect("diesel pool: failed to get connection");
        let rows_deleted = diesel::delete(roles::table.filter(roles::id.eq(role_id))).execute(&mut conn)?;
        if rows_deleted == 0 {
            Err(diesel::result::Error::NotFound)
        } else {
            Ok(())
        }
    }
}
