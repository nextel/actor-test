use actix::{Actor, Context, Message, Handler, Addr, SyncContext, Recipient, ArbiterService};
use actix::dev::{ResponseChannel, MessageResponse};
use crate::actor_counter_state::{CounterStateActor, SetCounter, GetCounter};
use std::thread;

#[derive(Debug, Default, Serialize, Deserialize)]

pub struct CounterActor{
}

impl actix::Supervised for CounterActor {}

impl ArbiterService for CounterActor {
    fn service_started(&mut self, ctx: &mut Context<Self>) {
        println!("Service started");
    }
}


impl Actor for CounterActor{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("started counter actor");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("stopped counter actor");
    }


}


impl Handler<StartIncCounter> for CounterActor{
    type Result = ();

    fn handle(&mut self, msg: StartIncCounter, ctx: &mut Self::Context) -> Self::Result {

        let mut counter =0 ;
        loop{
            // self.counter_state_actor.do_send(SetCounter{
            //     counter
            // });
            counter = counter+1;
            // thread::sleep(std::time::Duration::from_secs(5));
            println!("counter {:?}",counter);
            let addr = CounterStateActor::from_registry();

            let result = addr.try_send(SetCounter { counter: counter.clone() }).unwrap();
            if counter == 999{
                break
            }
        }
    }
}
impl CounterActor{
    async fn handle(&self){

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartIncCounter{

}
impl Message for StartIncCounter{
        type Result = ();
}










