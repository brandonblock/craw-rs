use actix::Actor;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod actors;
mod handlers;
mod middleware;
mod models;

async fn health_check(db: web::Data<sqlx::PgPool>) -> HttpResponse {
    match sqlx::query("SELECT 1").execute(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("healthy"),
        Err(_) => HttpResponse::ServiceUnavailable().body("database unavailable"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password123@localhost/craw".to_string());

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create db");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate database");

    let chat_server = actors::ChatServer::new().start();
    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    println!("starting chat server at {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(chat_server.clone()))
            .app_data(web::Data::new(db_pool.clone()))
            .route("/health", web::get().to(health_check))
            .route("/register", web::post().to(handlers::register_handler))
            .route("/login", web::post().to(handlers::login_handler))
            .route("/echo", web::get().to(handlers::echo_handler))
            .service(
                web::scope("/chat")
                    .wrap(HttpAuthentication::bearer(middleware::auth_middleware))
                    .route("", web::get().to(handlers::chat_handler)),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}
