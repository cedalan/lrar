use actix_web::{web, get, post, HttpResponse, Error};
use crate::db::DbPool;
use crate::models::{Tenant, TenantResponse, Burn, BurnRequest, BurnResponse};
use crate::schema::tenants::dsl::tenants;
use crate::schema::burn::dsl::*;
use actix_web::error::ErrorInternalServerError;
use diesel::prelude::*;
use crate::utils::get_weekly_chore;

#[get("/tenants")]
pub async fn get_tenants(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let tenants_data = web::block(move || {
        let mut conn = pool.get().expect("Failed to get DB connection");
        tenants.load::<Tenant>(&mut conn)
    })
    .await
    .map_err(|e| {
        eprintln!("Blocking error: {:?}", e);
        ErrorInternalServerError("Error during blocking operation")
    })?
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ErrorInternalServerError("Error querying the database")
    })?;

    // Base URL for images
    let base_url = "http://localhost:3001/images/";

    let all_tenant_chores = get_weekly_chore();

    let response_data: Vec<TenantResponse> = tenants_data
        .into_iter()
        .zip(all_tenant_chores.into_iter())
        .map(|(tenant, weekly_chore)| TenantResponse {
            id: tenant.id,
            name: tenant.name,
            age: tenant.age,
            image_url: tenant
                .image
                .as_ref()
                .map(|filename| format!("{}{}", base_url, filename)),
            burn_count: tenant.burn_count,
            dishwasher_count: tenant.dishwasher_count,
            favorite_quote: tenant.favorite_quote,
            weekly_chore: weekly_chore,
        })
        .collect();

    Ok(HttpResponse::Ok().json(response_data))
}

#[post("/tenant_burns")]
pub async fn get_tenant_burns(burn_request: web::Json<BurnRequest>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    println!("Received request with id: {}", burn_request.id);
    let burn_receiver_id = burn_request.id;

    let burn_data = web::block(move || {
        let mut conn = pool.get().expect("Failed to get DB connection");
        burn.filter(receiver_id.eq(burn_receiver_id))
        .load::<Burn>(&mut conn)
    })
    .await
    .map_err(|e| {
        eprintln!("Blocking error: {:?}", e);
        ErrorInternalServerError("Error during blocking operation")
    })?
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ErrorInternalServerError("Error querying the database")
    })?;

    println!("Fetched burn data with length {}", burn_data.len());

    let mut response_data = Vec::new();
    for burn_ in burn_data {
        response_data.push(BurnResponse {
            id: burn_.id,
            reason: burn_.reason,
            receiver_id: burn_.receiver_id,
            giver_id: burn_.giver_id,
            created_at: burn_.created_at,
        });
    }

    println!("Converted to Burnresponse with length {}", response_data.len());

    Ok(HttpResponse::Ok().json(response_data))
}