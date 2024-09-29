use actix_web::{web, App, HttpServer, Responder};

const PORT: u16 = 3001;

async fn hello() -> impl Responder {
    "Hello, World!\n"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    println!("Listening on port {}...", PORT);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
    .bind(format!("127.0.0.1:{}", PORT))?
    .run()
    .await
}
