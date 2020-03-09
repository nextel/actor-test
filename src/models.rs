use crate::actor::CounterActor;
use actix::Addr;

pub struct AppState{
pub counter_actor:Addr<CounterActor>
}