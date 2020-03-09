use actix_web::HttpServer;
use actix::Actor;
use crate::models::AppState;
use actix_web::{middleware, App};

mod actor;
mod handlers;
mod models;

extern crate serde;
#[macro_use]
extern crate serde_derive;


#[actix_rt::main]
async fn main() {
    HttpServer::new(move || {
        let parser_actor_address = actor::CounterActor {
            counter: 0
        }.start();

        App::new()
            .data(AppState {
                counter_actor: parser_actor_address,
            })
            .service(handlers::get_counter)
            .service(handlers::set_counter)
    })
        .bind("127.0.0.1:8080")
        .unwrap()
        .run().await;



}
