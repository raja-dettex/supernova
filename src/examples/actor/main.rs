
use std::fmt::Debug;
use std::io::{Error, ErrorKind};
use std::sync::{ Arc};
use supernova::actor;
use supernova::actor::actor::ActorId;
use supernova::sender::sender::Sink;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use async_trait::async_trait;

use actor::{actor::{Actor, Message}, pool::ActorPool, context::ActorContext};
pub fn mailbox< A: Actor + Send + 'static+ Debug>(actor : ActorPool<A>, receiver: mpsc::Receiver<Message>) -> ActorContext<A>{ 
    
    let ctx = ActorContext::new(actor, receiver);
    ctx
}



/// use case of ping actor
#[derive(Debug)]
pub struct PingActor{ 
    Id: ActorId
}

impl  PingActor { 
    pub fn new(id : ActorId) -> Self{ 
        Self{Id: id}
    }
}

use log::info;

#[async_trait]
impl Actor for PingActor {
    
    fn id(&mut self) -> ActorId { 
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

mod tests { 
    
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use uuid::Uuid;
    use super::actor::pool::ActorPool;
    use super::PingActor;
    use futures::{stream::{self, TryStreamExt}, StreamExt};

    #[tokio::test]
    pub async fn testPool() { 
        let mut pool: ActorPool<PingActor> = ActorPool::new();
        for i in  1..10 { 
            pool.add_actor(Arc::new(Mutex::new(PingActor::new(uuid::Uuid::new_v4()))));
        }
        let ids = pool.actors.iter().map(|a| async { 
            let actor = a.lock().await;
            return actor.Id;
        });
        let stream_of_ids = stream::iter(ids);
        let ids: Vec<Uuid> = stream_of_ids.then(|id| id).collect().await;
        for id in ids.into_iter() { 
            println!("{id}");
        }        
        assert_eq!(pool.actors.len(), 9);
    }

    // #[tokio::test]
    // pub async fn test_find_next() { 
    //     let mut pool: ActorPool<PingActor<u32>> = ActorPool::new();
    //     for i in  1..10 { 
    //         pool.add_actor(Arc::new(Mutex::new(PingActor::new(i as u32))));
    //     }
    //     for i in 1..10 { 
    //         let a = pool.find_next().lock().await;
    //         println!("{}", a.Id);
    //     }
    // }
    

    #[tokio::test]
    pub async fn futures_iter_to_iter() { 
        let futurer_iter = (1..5).into_iter().map(|x|async move { 
            x
        });
        let stream_of_futures = stream::iter(futurer_iter);
        let results: Vec<i32> = stream_of_futures.then(|a| a).collect().await;
        for result in results.into_iter() { 
            println!("{result}")
        }
        
    }
}


use log::LevelFilter;
use uuid::Uuid;
#[tokio::test]
async fn test_bench() { 
    let (sender , receiver) = mpsc::channel(100);
    let mut pool: ActorPool<PingActor> = ActorPool::new();
    for i in  1..10 { 
        pool.add_actor(Arc::new(Mutex::new(PingActor::new(Uuid::new_v4()))));
    }
    let mut ctx = mailbox(pool, receiver);
    let handle = tokio::spawn(async move { 
        let a = ctx.start().await;
    });

    sender.send(Message::Ping).await;
    for i in 1..10 { 
        sender.send(Message::Custom(format!("message {i}"))).await;
    }
    handle.await.unwrap();
} 

pub struct TcpSink { 
    pub sender : tokio::sync::mpsc::Sender<Message>, 
    pub listener : tokio::net::TcpListener
}

impl TcpSink { 
    pub fn new(sender: mpsc::Sender<Message>, listener : TcpListener) -> Self { 
        Self { sender, listener}
    }
}

impl Sink for TcpSink {
    async fn send_and_get_ack(&mut self, message : Option<Message>) -> Result<(), supernova::sender::sender::SendError> {
        let mut buf = vec![0u8; 2048];
        loop  { 
            let mut stream = self.listener.accept().await.unwrap().0;
            let n = stream.read(&mut buf).await.unwrap();
            let msg = String::from_utf8_lossy(&buf[..n]);
            self.sender.send(Message::Custom(msg.to_string())).await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ()>{
    env_logger::builder().filter_level(LevelFilter::Debug).init();
    let (sender , receiver) = mpsc::channel(100);
    let mut pool: ActorPool<PingActor> = ActorPool::new();
    for i in  1..10 { 
        pool.add_actor(Arc::new(Mutex::new(PingActor::new(Uuid::new_v4())))).await;
    }
    let mut ctx = mailbox(pool, receiver);
    let handle = tokio::spawn(async move { 
        ctx.start().await;
    });
    
    //sender.send(Message::Ping).await;
        //let addr = Socker
        let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
        let mut sink = TcpSink::new(sender, listener);
        
        let sink_handle = tokio::spawn(async move { 
            sink.send_and_get_ack(None).await;
        });
        //sender.send(Message::Custom(format!("message {i}"))).await;
    
    handle.await.unwrap();
    sink_handle.await.unwrap();
    Ok(())
}
