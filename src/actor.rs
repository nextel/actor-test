use actix::{Actor, Context, Message, Handler, Addr};
use actix::dev::{ResponseChannel, MessageResponse};
use crate::actor_counter_state::{CounterStateActor,SetCounter};

pub struct CounterActor{
    pub counter_state_actor:Addr<CounterStateActor>
}

impl Actor for CounterActor{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("started counter actor");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("stopped");
    }


}


impl Handler<StartIncCounter> for CounterActor{
    type Result = ();

    fn handle(&mut self, msg: StartIncCounter, ctx: &mut Self::Context) -> Self::Result {

        let mut counter =0 ;
        loop{
            self.counter_state_actor.do_send(SetCounter{
                counter
            });
            counter = counter+1;
            println!("counter {:?}",counter);
            if counter == 99999{
                break
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartIncCounter{

}
impl Message for StartIncCounter{
        type Result = ();
}










