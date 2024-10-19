use super::actor::{Actor, ActorId, ActorSafe};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct ActorPool<A : Actor + Send + 'static> { 
    pub actors : Vec<ActorSafe<A>>,
    capacity: usize,
    load_map: HashMap<ActorId, u32>,
    next_index : usize
}

pub struct ErrExeceedCap;

impl<A:Actor + Send + 'static > ActorPool<A> { 
    pub fn new() -> Self { 
        Self { actors: Vec::with_capacity(9), capacity: 9, next_index: 0, load_map: HashMap::new()}
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
    pub fn update_load_map(&mut self, actorId : ActorId) { 
        let load = self.load_map.get(&actorId).unwrap();    
        self.load_map.insert(actorId, *load + 1);
    } 
    pub async fn add_actor(&mut self, actor: ActorSafe<A>) { 
        self.actors.push(actor.clone());
        self.load_map.insert(actor.clone().lock().await.id(), 0);
    }

    pub async fn find_next(&mut self) -> (Arc<Mutex<A>>) {
        let mut min = 121212 as u32; 
        let mut id= Uuid::new_v4();
        for (k, v) in self.load_map.clone() { 
            if v < min { 
                min = v;
                id = k;
            }
        }; 
        for actor in self.actors.clone() { 
            if id == actor.lock().await.id() { 
                return actor;
            }
        }
        
        self.actors.get(0).unwrap().clone()
        
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