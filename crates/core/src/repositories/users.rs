use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use crate::models::users::{CreateUserEntity, UpdateUserDto, UserModel};
use crate::pagination::{List, RequestPagination, ResponsePagination};
use crate::schema::users;

#[derive(Clone)]
pub struct UserRepository {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl UserRepository {
    pub fn create(&self, entity: CreateUserEntity) -> Result<UserModel, diesel::result::Error> {
        let mut conn = self.pool.get().expect("diesel pool: failed to get connection");
        diesel::insert_into(users::table)
            .values(&entity)
            .get_result::<UserModel>(&mut conn)
    }

    pub fn get_one(&self, user_id: Uuid) -> Result<UserModel, diesel::result::Error> {
        let mut conn = self.pool.get().expect("diesel pool: failed to get connection");
        users::table.find(user_id).get_result::<UserModel>(&mut conn)
    }

    pub fn get_by_name(&self, name: &str) -> Result<UserModel, diesel::result::Error> {
        let mut conn = self.pool.get().expect("diesel pool: failed to get connection");
        users::table
            .filter(users::name.eq(name))
            .first::<UserModel>(&mut conn)
    }

    pub fn get_list(
        &self,
        pagination: ResponsePagination,
    ) -> Result<List<UserModel>, diesel::result::Error> {
        let mut conn = self.pool.get().expect("diesel pool: failed to get connection");

        let total_count: i64 = users::table.count().get_result(&mut conn)?;

        let items: Vec<UserModel> = users::table
            .limit(pagination.size)
            .offset((pagination.page.max(1) - 1) * pagination.size)
            .load::<UserModel>(&mut conn)?;

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
        user_id: Uuid,
        user_dto: UpdateUserDto,
    ) -> Result<UserModel, diesel::result::Error> {
        let mut conn = self.pool.get().expect("diesel pool: failed to get connection");
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(&user_dto)
            .get_result::<UserModel>(&mut conn)
    }

    pub fn delete(&self, user_id: Uuid) -> Result<(), diesel::result::Error> {
        let mut conn = self.pool.get().expect("diesel pool: failed to get connection");
        let rows_deleted = diesel::delete(users::table.filter(users::id.eq(user_id))).execute(&mut conn)?;
        if rows_deleted == 0 {
            Err(diesel::result::Error::NotFound)
        } else {
            Ok(())
        }
    }

    pub fn change_password(
        &self,
        user_id: Uuid,
        new_password_hash: String,
    ) -> Result<UserModel, diesel::result::Error> {
        let mut conn = self.pool.get().expect("diesel pool: failed to get connection");
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(users::password_hash.eq(new_password_hash))
            .get_result::<UserModel>(&mut conn)
    }
}
