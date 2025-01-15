use actix::Actor;
use actix_web::{web, App, HttpResponse, HttpServer};

mod actors;
mod handlers;

async fn health_check() -> HttpResponse {
    // TODO: Actually check for health (db connection, etc)
    HttpResponse::Ok().body("okeydokey")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = actors::ChatServer::new().start();
    // TODO: get this from env/startup args
    let bind_address = "127.0.0.1:8080";
    println!("starting chat server at {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(chat_server.clone()))
            .route("/health", web::get().to(health_check))
            .route("/echo", web::get().to(handlers::echo_handler))
            .route("/chat", web::get().to(handlers::chat_handler))
    })
    .bind(bind_address)?
    .run()
    .await
}
