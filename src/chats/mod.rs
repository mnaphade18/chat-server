use actix_web::{ web, get, post, Responder, Scope, HttpResponse };
use serde::{ Serialize, Deserialize };

use crate::models::chat::Chat;
use crate::models::message::{ Message, NewMessageInput };
use crate::state::AppState;
use crate::models::user::User;

#[derive(Serialize, Debug)]
struct UsersResponse {
    users: Vec<User>,
}

#[get("/users")]
async fn get_users(app_state: web::Data<AppState> ) -> impl Responder {
    let users = &app_state.users;

    HttpResponse::Ok()
        .json(users)
}

#[derive(Deserialize)]
struct NewUserInput {
    name: String,
}

#[post("/users/add")]
async fn add_user(body: web::Json<NewUserInput>, app_state: web::Data<AppState>) -> impl Responder {

    let new_user = User::new(body.into_inner().name);

    let response;
    {
        let mut users = app_state.users.lock().unwrap();

        users.push(new_user);
        response = users.last().unwrap().clone();
    }

    HttpResponse::Ok()
        .json(response)
}

#[get("/groups/{user_id}")]
async fn user_chats(path: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let chats = app_state.chats.lock().unwrap();

    let user_chats: Vec<&Chat> = chats.iter().filter(|c| c.users.iter().any(|uid| *uid == user_id)).collect();

    HttpResponse::Ok()
        .json(user_chats)
}

#[get("/messages/{group_id}/{user_id}")]
async fn get_messages(path: web::Path<(String, String)>, app_state: web::Data<AppState>) -> impl Responder {
    let (group_id, user_id) = path.into_inner();

    {
        let chats = app_state.chats.lock().unwrap();

        let user_chat = chats.iter().any(|c| c.id == group_id && c.users.iter().any(|uid| *uid == user_id));

        if !user_chat {
            return HttpResponse::NotFound()
                .body("No such chat group exists");
        }
    }

    let messages = app_state.messages.lock().unwrap();
    let user_messages: Vec<&Message> = messages.iter().filter(|m| m.group_id == group_id).collect();

    HttpResponse::Ok()
        .json(user_messages)
}

#[post("/messages/add")]
async fn add_message(body: web::Json<NewMessageInput>, app_state: web::Data<AppState>) -> impl Responder {
    let new_message = Message::new(body.into_inner());

    let mut messages = app_state.messages.lock().unwrap();
    messages.push(new_message.clone());

    HttpResponse::Ok()
        .json(new_message)
}

pub fn chats_scope() -> Scope {
    Scope::new("/chats")
        .service(get_users)
        .service(add_user)
        .service(user_chats)
        .service(get_messages)
        .service(add_message)
}
