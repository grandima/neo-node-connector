mod capability;
mod command;
mod message;
mod neoi64;
mod user_agent;
mod version_payload;

use bincode::enc::EncoderImpl;
use bincode::Encode;
use command::*;
use command::*;
use log::info;
use message::Message as NEOMessage;
use std::error::Error;
use std::hash::Hash;
use std::{io, result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, oneshot};
use tokio::time::*;
use tokio_native_tls::{native_tls, TlsConnector};
async fn write(stream: &mut TcpStream, message: message::Message) -> io::Result<usize> {
    let config = bincode::config::standard().with_fixed_int_encoding();
    let v = bincode::encode_to_vec(message, config).unwrap();
    stream.write(&v).await
}
fn print_result(result: &io::Result<usize>) {
    match result {
        Ok(size) => {
            println!("written: {:?}", size);
        }
        Err(error) => {
            println!("error writing: {:?}", error);
        }
    };
}
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let config = bincode::config::standard().with_fixed_int_encoding();
    let mut stream = TcpStream::connect("127.0.0.1:11333").await.unwrap();
    tokio::spawn(async move {
        let write_result = write(&mut stream, NEOMessage::default()).await;
        print_result(&write_result);
        let mut buffer = Vec::new();
        loop {
            let read_size = stream.read_buf(&mut buffer).await.unwrap_or_else(|error| {
                println!("Read error: {:?}", error);
                0
            });
            println!("read size: {:?}", read_size);
            if read_size == 0 {
                return;
            }
            let (mut message, length) = NEOMessage::try_deserialize(&buffer);
            let Some(mut message) = message.take() else {
            continue
         };
            _ = buffer.drain(0..length as usize);
            match message.command() {
                Command::Version => {
                    let result = write(&mut stream, message::Message::new(Command::Verack)).await;
                    print_result(&result);
                }
                Command::Verack => {
                    println!("Received Verack!!");
                }
                _ => {}
            };
        }
    })
    .await
    .unwrap();
    Ok(())
}
