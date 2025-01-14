use actix_web::{web, App, HttpServer};

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: get this from env/startup args
    let bind_address = "127.0.0.1:8080";
    println!("starting echo server at {}", bind_address);

    HttpServer::new(|| App::new().route("/ws", web::get().to(handlers::ws_handler)))
        .bind(bind_address)?
        .run()
        .await
}
