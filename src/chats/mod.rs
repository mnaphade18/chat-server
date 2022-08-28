pub mod socket;

use serde::{ Serialize, Deserialize };
use actix_web::{ web, get, post, http::header, error, Responder, Scope, HttpResponse, HttpRequest };
use actix_web_actors::ws;

use crate::models::chat::Chat;
use crate::models::message::{ Message, NewMessageInput };
use crate::state::AppState;
use crate::models::user::User;
use socket::Socket;

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

#[get("/messages/{group_id}")]
async fn get_messages(req: HttpRequest, path: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder {
    let group_id = path.into_inner();
    let user_id = req.headers()
        .get(header::HeaderName::from_static("x-user-id")).unwrap()
        .to_str().unwrap()
        .to_string();

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

    drop(messages);

    let active_users = app_state.active_users.lock().unwrap();
    let chats = app_state.chats.lock().unwrap();
    let group = chats.iter().find(|c| *c.id == new_message.group_id).unwrap();

    for user_id in group.users.iter() {
        if let Some(addr) = active_users.get(user_id) {
            println!("sending message to user id {user_id}");

            addr.do_send(new_message.clone());
        }
    }

    drop(active_users);
    drop(chats);

    HttpResponse::Ok()
        .json(new_message)
}

#[get("/messages/{group_id}/{user_id}/ws")]
async fn connect_group(req: HttpRequest, stream: web::Payload, path: web::Path<(String, String)>, app_state: web::Data<AppState>) -> impl Responder {
    let (group_id, user_id) = path.into_inner();

    {
        let groups = app_state.chats.lock().unwrap();

        if !groups.iter().any(|g| g.id == group_id && g.users.iter().any(|uid| *uid == user_id)) {
            return Err(error::ErrorBadRequest("Invalid group"))
        }
    }

    let (addr, response) = ws::WsResponseBuilder::new(Socket {}, &req, stream)
        .start_with_addr()
        .unwrap();

    let mut active_users = app_state.active_users.lock().unwrap();
    active_users.insert(user_id, addr);

    return Ok(response);
}

pub fn chats_scope() -> Scope {
    Scope::new("/chats")
        .service(get_users)
        .service(add_user)
        .service(user_chats)
        .service(get_messages)
        .service(add_message)
        .service(connect_group)
}
