use actix::prelude::*;
use std::collections::HashMap;

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Connect {
    pub addr: Recipient<ChatMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct ChatMessage {
    pub msg: String,
    pub id: usize,
}

pub struct ChatServer {
    sessions: HashMap<usize, Recipient<ChatMessage>>,
    next_id: usize,
}

impl ChatServer {
    pub fn new() -> Self {
        ChatServer {
            sessions: HashMap::new(),
            next_id: 0,
        }
    }

    fn broadcast_message(&self, message: &str, id: usize) {
        let msg = ChatMessage {
            msg: message.to_owned(),
            id,
        };

        for recipient in self.sessions.values() {
            recipient.do_send(msg.clone());
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    // returns the assigned client ID
    type Result = usize;

    fn handle(&mut self, msg: Connect, _ctx: &mut Context<Self>) -> Self::Result {
        // generate new client id
        let id = self.next_id;
        self.next_id += 1;

        // store the client's recipient
        self.sessions.insert(id, msg.addr);

        // broadcast join message to existin clients
        let join_msg = format!("User {} has joined", id);
        self.broadcast_message(&join_msg, id);

        id
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Context<Self>) -> Self::Result {
        self.sessions.remove(&msg.id);
        let disconnect_message = format!("User {} has left.", msg.id);
        self.broadcast_message(&disconnect_message, msg.id);
    }
}

impl Handler<ChatMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let chat_message = format! {"{}: {}", msg.id, msg.msg};
        self.broadcast_message(&chat_message, msg.id);
    }
}
