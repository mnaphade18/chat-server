use std::sync::Mutex;
use actix_web::web::Data;
use uuid::Uuid;

use crate::models::user::User;
use crate::models::message::Message;
use crate::models::chat::Chat;

pub struct AppState {
    pub users: Mutex<Vec<User>>,
    pub messages: Mutex<Vec<Message>>,
    pub chats: Mutex<Vec<Chat>>,
}

impl AppState {
    pub fn new () -> Data<Self> {
        let demo_user = User { id: "a3e1263a-999e-4f45-a145-a184636def21".to_owned(),
            name: "tanmay".to_owned(),
        };
        let demo_chat = Chat { id: "c6271f7a-264e-4049-a762-4f2b6ca8039e".to_owned(),
            name: "Test Chat 1".to_string(),
            users: vec![demo_user.id.clone()],
        };

        Data::new( Self { 
            users: Mutex::new(vec![demo_user]),
            messages: Mutex::new(vec![]),
            chats: Mutex::new(vec![demo_chat]),
        })   
    }
}
