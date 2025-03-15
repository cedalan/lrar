use actix_web::{web, get, HttpResponse, Error, post, patch, delete};
use crate::db::DbPool;
use crate::models::{Burn, BurnDto, BurnResponse, Note, NewNote, NewTenant, Tenant, TenantResponse};
use crate::schema::tenants::dsl::{tenants, id as tenant_id_column};
use crate::schema::burn::dsl::*;
use crate::schema::notes::dsl::{notes, id as note_id_column};
use actix_web::error::ErrorInternalServerError;
use diesel::prelude::*;
use crate::utils::{get_weekly_chore, give_burn_to_tenant, id_to_name, increase_dishwasher_count, insert_new_burn, insert_new_note, insert_new_tenant};

use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::fs;
use std::io::Write;
use uuid::Uuid;

#[patch("/dishwasher_count/{tenant_id}")]
pub async fn increment_tenant_dishwasher_count(tenant_id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    println!("Request recieved for increment_tenant_dishwasher_count: {}", tenant_id);

    let tenant_id = tenant_id.into_inner();

    let was_updated = web::block(move || {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");

        increase_dishwasher_count(&mut conn, tenant_id)
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

    // If no tenant was updated, we can assume that no tenant with that id exists!
    if was_updated == 0 {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Tenant not found"
        })));
    }

    // Otherwise success
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Dishwasher count incremented"
    })))
}

#[post("/burn")]
pub async fn create_burn(pool: web::Data<DbPool>, new_burn: web::Json<BurnDto>) -> Result<HttpResponse, Error> {
    println!("Request received for create_burn: {:?}", new_burn);

    let burn_data = new_burn.into_inner();
    let burned_tenant_id = burn_data.receiver_id;

    let result = web::block(move || {
        let mut conn = pool.get().expect("Failed to get DB connection from pool");

        // Check if tenant exists
        let tenant_exists: Option<Tenant> = tenants
            .filter(tenant_id_column.eq(burned_tenant_id))
            .first::<Tenant>(&mut conn)
            .optional()
            .expect("Error checking if tenant exists");

        if tenant_exists.is_none() {
            return Err(diesel::result::Error::NotFound);
        }

        // Insert the new burn
        let inserted_burn = insert_new_burn(&mut conn, burn_data)?;

        // Increase burn count
        give_burn_to_tenant(&mut conn, burned_tenant_id)?;

        Ok(inserted_burn)
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

    println!("Burn successfully inserted and tenant's burn count updated: {:?}", result);

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

#[post("/tenant_image")]
pub async fn upload_tenant_image(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut saved_filename = String::new();

    while let Some(field) = payload.next().await {
        let mut field = field?;
        let content_type = field
            .content_type()
            .map(|mime| mime.to_string())
            .unwrap_or_else(||"application/octet-stream".to_string());

        println!("Recieved content type: {}", content_type);

        let allowed_types = vec!["image/jpeg", "image/png", "image/jpg"];
        if !allowed_types.contains(&&content_type.as_str()) {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid file type. Only JPG and PNG are allowed!"
            })));
        }

        let file_extension = match content_type.as_str() {
            "image/png" => "png",
            "image/jpeg" | "image/jpg" => "jpg",
            _ => "dat", //just in case something sneaks through
        };

        //Generates a random filename - for simplicity
        let filename = format!("{}.{}", Uuid::new_v4(), file_extension); 
        let filepath = format!("assets/tenants_images/{}", filename);

        let mut file = fs::File::create(&filepath).expect("Failed to create file!!");

        while let Some(chunk) = field.next().await {
            let data = chunk?;

            file.write_all(&data);
        }

        saved_filename = filename.clone();
    }

    if saved_filename.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No file uploaded"
        })));
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "filename": saved_filename
    })))
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

#[delete("/note/{note_id}")]
pub async fn delete_note(note_id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let note_id = note_id.into_inner();

    let deleted_note = web::block(move || {
        let mut conn = pool.get().expect("Failed to get DB connection");

        let existing_note = notes
            .filter(note_id_column.eq(note_id))
            .first::<Note>(&mut conn)
            .optional()?;

        if existing_note.is_none() {
            return Err(diesel::result::Error::NotFound);
        }

        diesel::delete(notes.filter(note_id_column.eq(note_id))).execute(&mut conn)
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

    if deleted_note == 0 {
        println!("Note with ID {} not found", note_id);
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Note not found"
        })));
    }

    println!("Note with ID {} successfully deleted", note_id);
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Note deleted successfully"
    })))
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
        return Ok(HttpResponse::NoContent().finish());
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

    let all_tenant_chores = get_weekly_chore(tenants_data.len()); 

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
            current_burn_count: tenant.current_burn_count,
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