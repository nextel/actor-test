use actix_web::HttpServer;
use actix::Actor;
use crate::models::AppState;
use actix_web::{middleware, App};
use crate::actor_counter_state::CounterStateActor;

mod actor;
mod handlers;
mod models;
mod actor_counter_state;

extern crate serde;
#[macro_use]
extern crate serde_derive;


#[actix_rt::main]
async fn main() {
    let counter_state_actor_address = CounterStateActor {
        counter: 0
    }.start();

    let counter_actor_address = actor::CounterActor {
        counter_state_actor: counter_state_actor_address.clone()
    }.start();


    HttpServer::new(move || {


        App::new()
            .data(AppState {
                counter_actor: counter_actor_address.clone(),
                counter_state_actor: counter_state_actor_address.clone()
            })
            .service(handlers::get_counter)
            .service(handlers::set_counter)
    }).workers(1)
        .bind("127.0.0.1:8080")
        .unwrap()
        .run().await;



}
