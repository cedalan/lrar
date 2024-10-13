use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::Serialize;

const PORT: u16 = 3001;

#[derive(Serialize)]
struct Tenant {
    name: String,
    image_path: String,
    burns: u8,
}

async fn hello() -> impl Responder {
    "Hello, World!\n"
}

#[get("/tenants")]
async fn get_tenants() -> impl Responder {
    let my_tenant = Tenant {
        name: "Tenant 1".to_string(),
        image_path: "frontend/src/assets/LRAR_logo.jpg".to_string(),
        burns: 1,
    };

    HttpResponse::Ok().json(my_tenant)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    println!("Listening on port {}...", PORT);

    HttpServer::new(|| {
        App::new()
            // Configure CORS
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173") // Allow requests from this origin
                    .allowed_methods(vec!["GET", "POST"]) // Allow specific HTTP methods
                    .allow_any_header()
            )
            .route("/", web::get().to(hello))
            .service(get_tenants)
    })
    .bind(format!("127.0.0.1:{}", PORT))?
    .run()
    .await
}