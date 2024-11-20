use actix_web::{web, get, App, HttpResponse, HttpServer, Responder, Error};
use actix_cors::Cors;
use dotenv::dotenv;
use env_logger::Env;

mod db;
mod models;
mod schema;

use db::DbPool;
use models::Tenant;
use crate::schema::tenants::dsl::*;
use actix_web::error::ErrorInternalServerError;
use diesel::prelude::*;

const PORT: u16 = 3001;

async fn hello() -> impl Responder {
    "Hello, World!\n"
}

#[get("/tenants")]
async fn get_tenants(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let fetched_tenants = web::block(move || {
        let mut conn = pool.get().expect("Failed to get DB connection");
        tenants
            .select(Tenant::as_select())
            .load::<Tenant>(&mut conn)
    })
    .await
    .map_err(|e| {
        eprintln!("Blocking error: {:?}", e);
        ErrorInternalServerError("Error during blocking operation")
    })?;

    let fetched_tenants = fetched_tenants.map_err(|e| {
        eprintln!("Database error: {:?}", e);
        ErrorInternalServerError("Error fetching tenants from the database")
    })?;

    Ok(HttpResponse::Ok().json(fetched_tenants))
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
