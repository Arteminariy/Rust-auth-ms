// @generated automatically by Diesel CLI.

diesel::table! {
    roles (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        role_id -> Nullable<Uuid>,
        password_hash -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    roles,
    users,
);
