use actix_web::HttpServer;
use actix::{Actor, SyncArbiter};
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

        CounterStateActor {
            counter: 0
        }.start();



          actor::CounterActor {
        }.start();



    HttpServer::new(move || {


        App::new()
            .data(AppState {
            })
            .service(handlers::get_counter)
            .service(handlers::set_counter)
    }).workers(1)
        .bind("127.0.0.1:8080")
        .unwrap()
        .run().await;



}
