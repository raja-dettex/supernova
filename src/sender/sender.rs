use std::future::Future;

use crate::actor::actor::Message;

pub struct SendError;

pub trait Sink { 
    fn send_and_get_ack(&mut self, message: Option<Message>) -> impl Future<Output = Result<(), SendError>> ;
}


struct TestSender { 
    sender : tokio::sync::mpsc::Sender<Message>
}

impl Sink for TestSender { 
    async fn send_and_get_ack(&mut self, message:  Option<Message>) -> Result<(), SendError> {
        if message.is_none() { 
            return Ok(())
        }
        self.sender.send(message.unwrap()).await.map_err(|_| SendError)
    }
}



#[tokio::test]
pub async fn test_send_and_get_ack() { 
    let (mut sender, mut receiver) = tokio::sync::mpsc::channel(100);
    let mut t_sender = TestSender { sender};
    let handle = tokio::spawn(async move { 
      let mut i = 1;
      while i <= 10 { 
        let Some(msg) = receiver.recv().await else { 
            continue;
        };
        println!("message received : {msg:?}");
        i += 1;

      }  
    });
    let sender_handle = tokio::spawn(async move { 
        for i in 1..=10 { 
            match t_sender.send_and_get_ack(Some(Message::Custom(format!("message: {i}")))).await {
                Ok(_) => {},
                Err(_) => {},
            }
        }
    });
    handle.await.unwrap();
    sender_handle.await.unwrap();
}