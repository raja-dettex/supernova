
use std::{fmt::Debug, sync::Arc};
use log::info;


use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

pub type ActorSafe<A: Actor + Send + 'static> = Arc<Mutex<A>>;
pub type ActorId = Uuid;

#[async_trait]
pub trait Actor{
    fn started(&mut self);

    fn stopped(&mut self);

    async fn handle(&mut self, msg : Message);
    fn id(&mut self) -> ActorId;
} 



#[derive(Debug)]
pub enum Message { 
    Ping,
    Custom(String)
}