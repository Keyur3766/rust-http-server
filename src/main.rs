use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// A simple handler function that returns a welcome message
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello from Keyur!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}