use actix_web::HttpServer;
use actix::{Actor, SyncArbiter};
use crate::models::AppState;
use actix_web::{middleware, App};
use crate::actor_counter_state::CounterStateActor;
use actix_rt::Arbiter;

mod actor;
mod handlers;
mod models;
mod actor_counter_state;

extern crate serde;
#[macro_use]
extern crate serde_derive;


#[actix_rt::main]
async fn main() {
    let arbiter1 = Arbiter::new();
    let arbiter2 = Arbiter::new();



    Actor::start_in_arbiter(&arbiter1,|_|{
        CounterStateActor {
            counter: 0
        }
    });



    Actor::start_in_arbiter(&arbiter2,|_|{
            actor::CounterActor {
            }
        });





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
