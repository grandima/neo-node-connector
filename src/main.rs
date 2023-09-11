mod VersionPayload;
use VersionPayload::*;

use std::collections::HashMap;
use bincode::enc::EncoderImpl;
use bincode::Encode;
use reqwest::header::HeaderMap;
use tokio::net::TcpStream;
use tokio_native_tls::{TlsConnector, native_tls};
use log::{info};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("Starting application.");
    // Connect to the server using plain TCP
    let version = UserAgent::from("aaa".to_string());
    let config = bincode::config::standard();

    let v = bincode::encode_to_vec(version, config).unwrap();

    // let stream = TcpStream::connect("localhost:10333").await?;

    println!("{:?}", v);

    Ok(())
}
