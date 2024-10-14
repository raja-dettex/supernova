
use std::fmt::Debug;

use super::{pool::ActorPool, actor::{Actor, Message}};
use tokio::sync::mpsc;
pub struct ActorContext< A: Actor + Send + 'static>  {
    actors : ActorPool<A>,
    receiver : mpsc::Receiver<Message>
}


impl< A: Debug + Actor + Send + 'static>  ActorContext<A> { 
    pub fn new(actors:  ActorPool<A> , receiver : mpsc::Receiver<Message>)  -> Self { 
        Self {actors, receiver}
    }

    pub async fn start(&mut self) { 
        
        
            
             let mut receiver = std::mem::replace(&mut self.receiver, mpsc::channel(1).1);
            while let Some(msg) = receiver.recv().await { 
                let mut actor = self.actors.find_next();
                let actor_clone = actor.clone();
                let handle = tokio::spawn(async move { 
                    
                    actor_clone.lock().await.handle(msg).await;
                });
                let _ = handle.await;
            }
        
        
    }
}
