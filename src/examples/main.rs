use tokio::{io::AsyncWriteExt, net::TcpStream};


#[tokio::main]
async fn main() -> Result<(), ()>{ 
    for i in 1..=100 { 
        let mut stream = TcpStream::connect("0.0.0.0:8080").await.unwrap();
        let n = stream.write(format!("hello {i}").as_bytes()).await.unwrap();
        println!("bytes written : {n}");
        stream.shutdown();
    }
    Ok(())
}