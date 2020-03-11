use crate::actor::CounterActor;
use actix::Addr;
use crate::actor_counter_state::CounterStateActor;

pub struct AppState{
pub counter_actor:Addr<CounterActor>,
    pub counter_state_actor:Addr<CounterStateActor>,

}