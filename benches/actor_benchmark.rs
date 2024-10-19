use std::{fmt::Debug, sync::Arc};

use async_trait::async_trait;
use supernova::actor::{actor::{Actor, ActorSafe, Message}, context::ActorContext, pool::ActorPool};
/// use case of ping actor
#[derive(Debug)]
pub struct PingActor { 
    Id: Uuid
}

impl PingActor { 
    pub fn new(id : Uuid) -> Self{ 
        Self{Id: id}
    }
}

use log::info;
use tokio::sync::{mpsc::{self, Sender}, Mutex};

#[async_trait]
impl Actor for PingActor {
    fn id(&mut self) -> Uuid { 
        self.Id
    }

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


fn setup_pool_and_sender() -> (ActorContext<PingActor>, Sender<Message>) { 
    let (sender, receiver) = mpsc::channel(20);
    let mut pool: ActorPool<PingActor> = ActorPool::new();
    for i in 1..10 { 
        pool.add_actor(Arc::new(Mutex::new(PingActor::new(Uuid::new_v4()))));
    }
    let ctx = ActorContext::new(pool, receiver);
    (ctx , sender)
}


use bencher::{benchmark_group, benchmark_main, Bencher};
use uuid::Uuid;


pub fn actor_benchmark(bencher : &mut Bencher) { 
    
    // c.warm_up_time(std::time::Duration::from_secs(1));
    
    bencher.iter(|| { 
        let (mut ctx , sender ) = setup_pool_and_sender();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let h = rt.block_on(async move { 
            let handle = tokio::spawn(async move { 
                ctx.start().await;
            });
            
            sender.send(Message::Custom(format!("message : 1"))).await.unwrap();
            
            handle.await.unwrap();
        });
    });
    
}

pub fn test_bench(bencher : &mut Bencher) { 
    bencher.iter(|| { 
        for i in 1..=10 { 
            println!("{}", i);
        }
    })
}

benchmark_group!(benches, test_bench);
benchmark_main!(benches);