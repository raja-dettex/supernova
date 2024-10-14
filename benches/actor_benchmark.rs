use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use supernova::actor::{actor::{Actor, ActorSafe, Message}, context::ActorContext, pool::ActorPool};
/// use case of ping actor
#[derive(Debug)]
pub struct PingActor<id : std::marker::Send + Debug > { 
    Id: id
}

impl<id: Send + Debug>  PingActor<id> { 
    pub fn new(id : id) -> Self{ 
        Self{Id: id}
    }
}

use log::info;
use tokio::sync::{mpsc::{self, Sender}, Mutex};

#[async_trait]
impl<id: std::marker::Send + Debug> Actor for PingActor<id> {

    
    fn started(&mut self) { 
        info!("actor {:?} started", self.Id);
    }

    fn stopped(&mut self) { 
        info!("actor {:?} stopped", self.Id);
    }
    
    async fn handle(&mut self,msg:Message) {
        match msg  { 
            Message::Ping => info!("Actor : {:?} , Message: {:?} received,", self.Id, msg ),
            Message::Custom(str) => info!("Actor : {:?} other message : {str} received", self.Id)
        }
    }
}


fn setup_pool_and_sender() -> (ActorContext<PingActor<u32>>, Sender<Message>) { 
    let (sender, receiver) = mpsc::channel(20);
    let mut pool: ActorPool<PingActor<u32>> = ActorPool::new();
    for i in 1..10 { 
        pool.add_actor(Arc::new(Mutex::new(PingActor::new(i as u32))));
    }
    let ctx = ActorContext::new(pool, receiver);
    (ctx , sender)
}


use criterion::{Criterion, criterion_group, criterion_main};


pub fn actor_benchmark(c : &mut Criterion) { 
    
    c.bench_function("actor benchmark", |b| { 
        b.iter(|| { 
            let (mut ctx , sender ) = setup_pool_and_sender();
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move { 
                let handle = tokio::spawn(async move { 
                    ctx.start().await;
                });
                
                sender.send(Message::Custom(format!("message : 1"))).await.unwrap();
                
                handle.await.unwrap();
            })
        });
    });
}

criterion_group!(benches, actor_benchmark);
criterion_main!(benches);