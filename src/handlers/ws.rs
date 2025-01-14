use actix_web::{web, Error, HttpRequest, HttpResponse};

pub async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}
