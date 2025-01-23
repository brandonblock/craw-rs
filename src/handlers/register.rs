use actix_web::{web, HttpResponse};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{RegisterRequest, User};

pub async fn register_handler(
    pool: web::Data<PgPool>,
    user_data: web::Json<RegisterRequest>,
) -> HttpResponse {
    //generate password hash
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(user_data.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let result = sqlx::query_as!(
        User,
        "INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3) RETURNING id, username, password_hash",
        Uuid::new_v4(),
        user_data.username,
        hash,
    ).fetch_one(pool.get_ref()).await;

    match result {
        Ok(user) => HttpResponse::Created().json(user),
        Err(_) => HttpResponse::BadRequest().body("Username alreay exists"),
    }
}
