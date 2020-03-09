use actix_web::{web, HttpResponse, Error, HttpRequest};
use actix_web::{get, post};
use crate::models::AppState;
use crate::actor::{GetCounter, SetCounter};


#[post("/set")]
pub async fn set_counter(
    set_request: web::Json<SetCounter>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let addr = data.counter_actor.clone();
    addr.try_send(SetCounter{ counter: set_request.counter }) ;
    Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json("{}"))

}
#[get("/get")]
pub async fn get_counter(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let addr = data.counter_actor.clone();
    let counter = addr.send(GetCounter{}).await;
         Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(counter.unwrap()))

}


