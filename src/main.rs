mod VersionPayload;
mod neoi64;
mod user_agent;
mod Capability;
mod Message;
mod Command;


use bincode::enc::EncoderImpl;
use bincode::Encode;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use tokio_native_tls::{TlsConnector, native_tls};
use log::{info};
use Message::Message as NEOMessage;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn Error>> {
   env_logger::init();

   let listener = TcpListener::bind("127.0.0.1:22333").await.unwrap();
   tokio::spawn(async move {
      let mut buffer = Vec::new();
      loop {
         match listener.accept().await {
            Ok((mut stream, addr)) => {
               let size = stream.read_buf(&mut buffer).await.unwrap();
               let message = NEOMessage::try_deserialize(&buffer);
               println!("{:?}", message.0.unwrap());
            },
            Err(error) => {
               println!("{:?}", error);
               return
            }
         }
      }
   }).await.unwrap();
   let mut config = bincode::config::standard().with_fixed_int_encoding();
   let m = NEOMessage::default();
   let v = bincode::encode_to_vec(m, config).unwrap();



   Ok(())
}
