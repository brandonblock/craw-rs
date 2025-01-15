use actix::{
    dev::ContextFutureSpawner, fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext,
    Handler, StreamHandler, WrapFuture,
};
use actix_web_actors::ws;

use super::chat_server::{ChatMessage, ChatServer, Connect, Disconnect};

pub struct ChatSession {
    id: Option<usize>,
    addr: Addr<ChatServer>,
}

impl ChatSession {
    pub fn new(addr: Addr<ChatServer>) -> Self {
        ChatSession { id: None, addr }
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, _ctx| {
                if let Ok(id) = res {
                    act.id = Some(id);
                }
                fut::ready(())
            })
            .wait(ctx);
    }
}

impl Handler<ChatMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        // forward the chat message to the WebSocket as text
        ctx.text(msg.msg);
    }
}

//TODO: actually handle incoming messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let msg = text.to_string();
                if let Some(id) = self.id {
                    self.addr.do_send(ChatMessage { msg, id });
                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(reason)) => {
                if let Some(id) = self.id {
                    self.addr.do_send(Disconnect { id });
                }
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        if let Some(id) = self.id {
            self.addr.do_send(Disconnect { id });
        }
    }
}
