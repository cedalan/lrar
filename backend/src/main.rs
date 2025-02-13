use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder};
use actix_cors::Cors;
use dotenv::dotenv;
use env_logger::Env;

mod db;
mod models;
mod schema;
mod endpoints;
mod utils;

use endpoints::{create_burn, create_note, create_tenant, delete_note, get_notes, get_tenant_burns, get_tenants};

const PORT: u16 = 3001;

async fn hello() -> impl Responder {
    "Hello, World!\n"
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
            .service(fs::Files::new("/images", "/lrar/backend/assets/tenants_images").show_files_listing())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST"])
                    .allow_any_header(),
            )
            .route("/", web::get().to(hello))
            .service(get_tenants)
            .service(get_tenant_burns)
            .service(get_notes)
            .service(create_burn)
            .service(create_tenant)
            .service(create_note)
            .service(delete_note)
    })
    .bind(format!("0.0.0.0:{}", PORT))?
    .run()
    .await
}
