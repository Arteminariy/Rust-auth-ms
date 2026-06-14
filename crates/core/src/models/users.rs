use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Debug, Queryable)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub role_id: Option<Uuid>,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserDto {
    pub name: String,
    pub role_id: Option<Uuid>,
    pub password: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct CreateUserEntity {
    pub name: String,
    pub role_id: Option<Uuid>,
    pub password_hash: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = users)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserDto {
    pub name: String,
    pub role_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: Uuid,
    pub name: String,
    pub role_id: Option<Uuid>,
}

impl From<UserModel> for UserDto {
    fn from(value: UserModel) -> Self {
        Self {
            id: value.id,
            name: value.name,
            role_id: value.role_id,
        }
    }
}
