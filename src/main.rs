mod VersionPayload;
mod neoi64;
mod user_agent;
mod Capability;
mod Message;

use user_agent::UserAgent;
use VersionPayload::*;

use std::collections::HashMap;
use bincode::enc::EncoderImpl;
use bincode::Encode;
use reqwest::header::HeaderMap;
use tokio::net::TcpStream;
use tokio_native_tls::{TlsConnector, native_tls};
use log::{info};
use Message::Message as NEOMessage;


 fn main()  {
    env_logger::init();

    info!("Starting application.");
    // Connect to the server using plain TCP

    let m = NEOMessage::new();
    let mut config = bincode::config::standard().with_fixed_int_encoding();
    println!("{:?}", m);
    let v = bincode::encode_to_vec(m, config).unwrap();

    // let stream = TcpStream::connect("localhost:10333").await?;

    println!("{:?}", v);
}
