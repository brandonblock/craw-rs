use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};

use crate::actors::{ChatServer, ChatSession};

pub async fn chat_handler(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    let chat_session = ChatSession::new(server.get_ref().clone());
    actix_web_actors::ws::start(chat_session, &req, stream)
}
