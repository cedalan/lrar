use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{tenants, burn, notes};

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = notes)]
pub struct Note {
    pub id: i32,
    pub message: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = notes)]
pub struct NewNote {
    pub message: String,
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = tenants)]
pub struct Tenant {
    pub id: i32,
    pub name: String,
    pub age: Option<i32>,
    pub image: Option<String>,
    pub burn_count: Option<i32>,
    pub dishwasher_count: Option<i32>,
    pub favorite_quote: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = tenants)]
pub struct NewTenant {
    pub name: String,
    pub age: Option<i32>,
    pub image: Option<String>,
    pub burn_count: Option<i32>,
    pub dishwasher_count: Option<i32>,
    pub favorite_quote: Option<String>,
}

#[derive(Serialize)]
pub struct TenantResponse {
    pub id: i32,
    pub name: String,
    pub age: Option<i32>,
    pub image_url: Option<String>,
    pub burn_count: Option<i32>,
    pub dishwasher_count: Option<i32>,
    pub favorite_quote: Option<String>,
    pub weekly_chore: String,
}

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = burn)]
pub struct Burn {
    pub id: i32,
    pub reason: String,
    pub receiver_id: i32,
    pub giver_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct BurnResponse {
    pub id: i32,
    pub reason: String,
    pub receiver_name: String,
    pub giver_name: String,
    pub created_at: NaiveDateTime,
}


#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = burn)]
pub struct BurnDto {
    pub id: Option<i32>,
    pub reason: String,
    pub receiver_id: i32,
    pub giver_id: i32,
    pub created_at: NaiveDateTime,
}