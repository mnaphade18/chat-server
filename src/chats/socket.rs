use actix::{ Actor, StreamHandler, Handler };
use actix_web_actors::ws::{ Message, ProtocolError, WebsocketContext };

use crate::models::message::Message as MessageModel;

pub struct Socket;

impl Actor for Socket {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Actor has started")
    }
}

impl StreamHandler<Result<Message, ProtocolError>> for Socket {
    fn handle(&mut self, message: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match message {
            Ok(Message::Ping(msg)) => ctx.pong(&msg),
            Ok(Message::Text(text)) => ctx.text(text),
            _ => println!("Some other message type received {:?}", message),
        }
    }
}


impl actix::Message for MessageModel {
    type Result = ();
}

impl Handler<MessageModel> for Socket {
    type Result = ();

    fn handle(&mut self, message: MessageModel, ctx: &mut Self::Context) {
        let message_string = serde_json::to_string(&message).unwrap();
        ctx.text(message_string)
    }
}
