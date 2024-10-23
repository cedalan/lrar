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
    const N_TENANTS:usize = 6;
    let mut tenants: Vec<Tenant> = Vec::with_capacity(N_TENANTS);
    
    for x in 0..N_TENANTS {
        let my_tenant = Tenant {
            name: "Tenant name".to_string(),
            image_path: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSaNYHA2New-SxlLuwl_vLacub2oAPtnplWGA&s".to_string(),
            burns: x as u8,
        };
        tenants.push(my_tenant)
    }

    HttpResponse::Ok().json(tenants)
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