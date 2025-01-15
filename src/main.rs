use actix_web::{web, App, HttpResponse, HttpServer};

mod actors;
mod handlers;

async fn health_check() -> HttpResponse {
    // TODO: Actually check for health (db connection, etc)
    HttpResponse::Ok().body("okeydokey")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: get this from env/startup args
    let bind_address = "127.0.0.1:8080";
    println!("starting echo server at {}", bind_address);

    HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/echo", web::get().to(handlers::echo_handler))
            .route("/chat", web::get().to(handlers::chat_handler))
    })
    .bind(bind_address)?
    .run()
    .await
}
