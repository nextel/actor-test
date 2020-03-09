use actix::{Actor, Context, Message, Handler};
use actix::dev::{ResponseChannel, MessageResponse};

pub struct CounterActor{
    pub counter :usize
}

impl Actor for CounterActor{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.set_mailbox_capacity(1)
    }
}


impl Handler<SetCounter> for CounterActor{
    type Result = ();

    fn handle(&mut self, msg: SetCounter, ctx: &mut Self::Context) -> Self::Result {
        println!("setCounter {:?}",msg.counter.clone());
        self.counter = msg.counter
    }
}


impl Handler<GetCounter> for CounterActor{
    type Result = GetCounterResp;

    fn handle(&mut self, msg: GetCounter, ctx: &mut Self::Context) -> Self::Result {
        println!("get counter {:?}",self.counter.clone());

        GetCounterResp{
            counter: self.counter.clone()
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct SetCounter{
    pub counter :usize
}

impl Message for SetCounter{
    type Result = ();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct GetCounter{

}

impl Message for GetCounter{
    type Result = GetCounterResp;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct GetCounterResp{
pub counter :usize
}

impl Message for GetCounterResp{
    type Result = ();
}


impl<A, M> MessageResponse<A, M> for GetCounterResp
    where
        A: Actor,
        M: Message<Result = GetCounterResp>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}





