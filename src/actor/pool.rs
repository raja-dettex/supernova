use super::actor::{Actor, ActorSafe};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ActorPool<A : Actor + Send + 'static> { 
    pub actors : Vec<ActorSafe<A>>,
    capacity: usize,
    next_index : usize
}

pub struct ErrExeceedCap;

impl<A:Actor + Send + 'static > ActorPool<A> { 
    pub fn new() -> Self { 
        Self { actors: Vec::with_capacity(9), capacity: 9, next_index: 0}
    }
    pub fn init_pool(&mut self , init_cap: usize, actor: ActorSafe<A>) -> Result<(), ErrExeceedCap> { 
        if init_cap > self.capacity { 
           return  Err(ErrExeceedCap);
        }
        for i in 1..init_cap { 
            self.add_actor(actor.clone());
        }
        Ok(())
    }

    pub fn add_actor(&mut self, actor: ActorSafe<A>) { 
        self.actors.push(actor);
    }

    pub fn find_next(&mut self) -> &(Arc<Mutex<A>>) { 
        if self.next_index >= self.capacity { 
            self.next_index = 1;
            return &self.actors[0];
        }
        
        let actor = &self.actors[self.next_index];
        self.next_index += 1;
        actor
    }

} 


// mod test {
//     use super::ActorPool;
    
//     use core::panic;
//     use std::sync::Arc;
//     use tokio::sync::Mutex;
//     #[tokio::test] 
//     pub async fn pool_enumerator() { 
//         let mut pool: ActorPool<PingActor<u32>> = ActorPool::new();
//         for i in 1..10  { 
//             pool.add_actor(Arc::new(Mutex::new(PingActor::new(i as u32))));
//         }    
//         let a = pool.find_next();
//         println!("{}", a.lock().await.Id);
//         let b = pool.find_next();
//         println!("{}", b.lock().await.Id);
//     }
// }