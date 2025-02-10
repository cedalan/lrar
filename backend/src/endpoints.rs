use actix_web::{web, get, HttpResponse, Error, post};
use crate::db::DbPool;
use crate::models::{Burn, BurnDto, BurnResponse, Note, NewNote, NewTenant, Tenant, TenantResponse};
use crate::schema::tenants::dsl::{tenants, id as tenant_id_column};
use crate::schema::burn::dsl::*;
use crate::schema::notes::dsl::*;
use actix_web::error::ErrorInternalServerError;
use diesel::prelude::*;
use crate::utils::{get_weekly_chore, id_to_name, insert_new_burn, insert_new_tenant, insert_new_note};


#[post("/burn")]
pub async fn create_burn(pool: web::Data<DbPool>, new_burn: web::Json<BurnDto>) -> Result<HttpResponse, Error> {
    println!("Request received for create_burn: {:?}", new_burn);
     let new_burn = web::block (move || {
        let mut conn = pool.get().expect("failed to get db connection from pool");
        insert_new_burn(&mut conn, new_burn.into_inner())
    }).await
        .map_err(|e| {
            eprintln!("Blocking error: {:?}", e);
            ErrorInternalServerError("Error during blocking operation")
        })
        .map_err(|e| {
            eprintln!("Database error: {:?}", e);
            ErrorInternalServerError("Error querying the database")
        })?;
    println!("Sucessfully inserted?");
    let result = new_burn.unwrap();
    println!("Burn successfully inserted: {:?}", result);
    Ok(HttpResponse::Ok().json(result))
}

#[post("/tenant")]
pub async fn create_tenant(pool: web::Data<DbPool>, new_tenant: web::Json<NewTenant>) -> Result<HttpResponse, Error> {
    println!("Request recieved for create_tenant: {:?}", new_tenant);
    let new_tenant = web::block(move || {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        insert_new_tenant(&mut conn, new_tenant.into_inner())
    }).await
    .map_err(|e| {
        eprintln!("Blocking error: {:?}", e);
        ErrorInternalServerError("Error during blocking operation")
    })
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ErrorInternalServerError("Error querying the database")
    })?;

    let result = new_tenant.unwrap();
    println!("Tenant successfully inserted: {:?}", result);

    Ok(HttpResponse::Ok().json(result))
}

#[post("/note")]
pub async fn create_note(pool: web::Data<DbPool>, new_note: web::Json<NewNote>) -> Result<HttpResponse, Error> {
    println!("Request recieved for create_note: {:?}", new_note);

    if new_note.message.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": "message cannot be empty"})));
    }

    let new_note = web::block(move || {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");
        insert_new_note(&mut conn, new_note.into_inner())
    }).await
    .map_err(|e| {
        eprintln!("Blocking error: {:?}", e);
        ErrorInternalServerError("Error during blocking operation")
    })
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ErrorInternalServerError("Error querying the database")
    })?;

    let result = new_note.unwrap();
    println!("Note successfully inserted: {:?}", result);

    Ok(HttpResponse::Ok().json(result))
}

#[get("notes")]
pub async fn get_notes(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let notes_data = web::block(move || {
        let mut conn = pool.get().expect("Failed to get DB connection");
        notes.load::<Note>(&mut conn)
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

    if notes_data.len() < 1 {
        return Ok(HttpResponse::NoContent().json(serde_json::json!({"error": "No notes were found"})));
    }

    Ok(HttpResponse::Ok().json(notes_data))
}

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

    let mut all_tenant_chores = get_weekly_chore();

    if tenants_data.len() > all_tenant_chores.len() {
        println!("Number of chores are less than the number of tenants. Adding empty chroe to avoid truncation");
        for _ in 0..(tenants_data.len() - all_tenant_chores.len()) {
            all_tenant_chores.push("Nothing".to_string());
        }
    } 

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

#[get("/tenants/{tenant_id}/burns")]
pub async fn get_tenant_burns(tenant_id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let tenant_id = tenant_id.into_inner();
    println!("Received request with id test: {}", tenant_id);

    let tenant_exists = web::block({
        let pool_clone = pool.clone();
        move || {
            let mut conn = pool_clone.get().expect("Failed to get DB connection");
            tenants.filter(tenant_id_column.eq(tenant_id)).first::<Tenant>(&mut conn).optional()
        }
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

    if tenant_exists.is_none() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Tenant not found!"
        })));
    }

    let burn_data = web::block({
        let pool_clone = pool.clone();
        move || {
            let mut conn = pool_clone.get().expect("Failed to get DB connection");
            burn.filter(receiver_id.eq(tenant_id)).load::<Burn>(&mut conn)
        }
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

    println!("Fetched burn data with length {}", burn_data.len());

    if burn_data.is_empty() {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "No burn data found for that user!"
        })));
    }

    let mut response_data = Vec::new();
    for burn_ in burn_data {
        response_data.push(BurnResponse {
            id: burn_.id,
            reason: burn_.reason,
            receiver_name: id_to_name(burn_.receiver_id, &tenants_data).await,
            giver_name: id_to_name(burn_.giver_id, &tenants_data).await,
            created_at: burn_.created_at,
        });
    }

    println!("Converted to Burnresponse with length {}", response_data.len());

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "count": response_data.len(),
        "data": response_data,
    })))
}