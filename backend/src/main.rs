use actix_files as fs;
use actix_web::{web, get, App, HttpResponse, HttpServer, Responder, Error};
use actix_cors::Cors;
use dotenv::dotenv;
use env_logger::Env;

mod db;
mod models;
mod schema;

use db::DbPool;
use models::{Tenant, TenantResponse};
use crate::schema::tenants::dsl::tenants;
use actix_web::error::ErrorInternalServerError;
use diesel::prelude::*;

const PORT: u16 = 3001;

async fn hello() -> impl Responder {
    "Hello, World!\n"
}

#[get("/tenants")]
async fn get_tenants(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
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

    let response_data: Vec<TenantResponse> = tenants_data
        .into_iter()
        .map(|tenant| TenantResponse {
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
        })
        .collect();

    Ok(HttpResponse::Ok().json(response_data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let pool = db::establish_connection();

    db::run_migrations(&pool);

    println!("Listening on port {}...", PORT);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(fs::Files::new("/images", "./backend/assets/tenants_images").show_files_listing())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST"])
                    .allow_any_header(),
            )
            .route("/", web::get().to(hello))
            .service(get_tenants)
    })
    .bind(format!("0.0.0.0:{}", PORT))?
    .run()
    .await
}
