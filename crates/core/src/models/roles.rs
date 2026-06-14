use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::roles;

#[derive(Debug, Queryable, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = roles)]
#[serde(rename_all = "camelCase")]
pub struct NewRole {
    pub name: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = roles)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRole {
    pub name: String,
}
