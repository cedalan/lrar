use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tenants)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tenant {
    pub name: String,
    pub height: i32,
    pub profile_picture_uri: String,
    pub burns: i32,
}