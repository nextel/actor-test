use actix_web::{web, HttpResponse, Error, HttpRequest};
use actix_web::{get, post};
use crate::models::AppState;
use crate::actor_counter_state::{SetCounter, GetCounter};
use crate::actor::StartIncCounter;


#[post("/set")]
pub async fn set_counter(
    set_request: web::Json<StartIncCounter>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let addr = data.counter_actor.clone();
    addr.try_send(StartIncCounter{}) ;
    Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json("{}"))

}
#[get("/get")]
pub async fn get_counter(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let addr = data.counter_state_actor.clone();
    let counter = addr.send(GetCounter{}).await;
         Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(counter.unwrap()))

}


