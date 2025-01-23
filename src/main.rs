use actix::Actor;
use actix_web::{web, App, HttpResponse, HttpServer};
use sqlx::postgres::PgPoolOptions;

mod actors;
mod handlers;
mod models;

async fn health_check(db: web::Data<sqlx::PgPool>) -> HttpResponse {
    match sqlx::query("SELECT 1").execute(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("healthy"),
        Err(_) => HttpResponse::ServiceUnavailable().body("database unavailable"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //spin up db
    //TODO: get from env/startup args
    let database_url = "postgres://postgres:password123@localhost/craw";

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to create db");

    let chat_server = actors::ChatServer::new().start();
    // TODO: get this from env/startup args
    let bind_address = "127.0.0.1:8080";
    println!("starting chat server at {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(chat_server.clone()))
            .app_data(web::Data::new(db_pool.clone()))
            .route("/health", web::get().to(health_check))
            .route("/echo", web::get().to(handlers::echo_handler))
            .route("/chat", web::get().to(handlers::chat_handler))
    })
    .bind(bind_address)?
    .run()
    .await
}
