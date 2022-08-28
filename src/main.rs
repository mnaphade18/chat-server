mod chats;
mod state;
mod models;

use actix_web::{ get, App, HttpServer, Responder, middleware };
use state::AppState;


#[get("/")]
async fn hello_route() -> impl Responder {
    "This api is working"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let app_state = AppState::new();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(app_state.clone())
            .service(hello_route)
            .service(chats::chats_scope())
    }).bind(("0.0.0.0", 4000)).unwrap().run().await
}
