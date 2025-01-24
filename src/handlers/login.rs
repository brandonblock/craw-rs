use actix_web::{web, HttpResponse};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sqlx::PgPool;

use crate::models::{User, LoginRequest};
use crate::middleware::generate_token;

pub async fn login_handler(
    pool: web::Data<PgPool>,
    login_data: web::Json<LoginRequest>,
) -> HttpResponse {
    let user_result = sqlx::query_as!(
        User,
        "SELECT id, username, password_hash FROM users WHERE username = $1",
        login_data.username,
    ).fetch_optional(pool.get_ref()).await;

    match user_result {
        Ok(Some(user)) => {
            let parsed_hash = match PasswordHash::new(&user.password_hash) {
                Ok(hash) => hash,
                Err(_) => {
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "error processing credentials"
                    }))
                }
            };

            if Argon2::default()
            .verify_password(login_data.password.as_bytes(), &parsed_hash)
            .is_ok()
            {
                match generate_token(user.id, user.username.clone()) {
                    Ok(token) => HttpResponse::Ok().json(serde_json::json!({
                        "token": token,
                        "user": {
                            "id": user.id,
                            "username": user.username
                        }
                    })),
                    Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to generate token"
                    })),
                }
            } else {
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "Invalid credentials"
                }))
            }
        }
        Ok(None) => {
            HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid credentials"
            }))
            
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Error processing login request"
            }))
        }
    }
}