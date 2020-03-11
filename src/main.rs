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


    let addr_state_actor = Actor::start_in_arbiter(&arbiter1, |_| {
        CounterStateActor {
            counter: 0
        }
    });

    let addr_cloned = addr_state_actor.clone();


    let addr_counter_actor = Actor::start_in_arbiter(&arbiter2, move|_| {
        actor::CounterActor {
            actor_state_counter_address: addr_cloned
        }
    });





    HttpServer::new( move || {


        App::new()
            .data(AppState {
                counter_actor: addr_counter_actor.clone(),
                counter_state_actor: addr_state_actor.clone()
            })
            .service(handlers::get_counter)
            .service(handlers::set_counter)
    }).workers(1)
        .bind("127.0.0.1:8080")
        .unwrap()
        .run().await;



}
