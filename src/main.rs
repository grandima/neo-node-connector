use std::collections::HashMap;
use reqwest::header::HeaderMap;
use tokio::net::TcpStream;
use tokio_native_tls::{TlsConnector, native_tls};
use log::{info};
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Read the self-signed certificate.
//     let certs = std::fs::read("/Users/grandima/Desktop/Local/Blockchain/docker-rippled/my_ssl/server.pem")?;
//
//     // Create a certificate object from the certificate bytes.
//     let cert = native_tls::Certificate::from_pem(&certs)?;
//
//     // Create a TlsConnector that trusts the self-signed certificate.
//     let mut builder = native_tls::TlsConnector::builder();
//     builder.add_root_certificate(cert);
//     let connector = TlsConnector::from(builder.build()?);
//
//     // Connect to the server with TLS.
//     let stream = TcpStream::connect("127.0.0.1:51235").await?;
//     let _tls_stream = connector.connect("127.0.0.1", stream).await?;
//
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("Starting application.");
    // Connect to the server using plain TCP
    let stream = TcpStream::connect("s2.ripple.com:51234").await?;


    let connector = native_tls::TlsConnector::new()?;
    let connector = TlsConnector::from(connector);

    let mut tls_stream = connector.connect("s2.ripple.com", stream).await?;

    println!("Successfully connected to the server over TLS!");
    let client = reqwest::Client::new();
    let response = client.post("https://s2.ripple.com:51235/")
        .header("User-Agent", "rippled-1.11.0+DEBUG")
        .header("Upgrade", "RTXP/1.2, XRPL/2.0")
        .header("Connection", "Upgrade")
        .header("Connect-As", "Peer")
        .header("Public-Key", "n94MvLTiHQJjByfGZzvQewTxQP2qjF6shQcuHwCjh5WoiozBrdpX")
        .header("Session-Signature", "MEUCIQCOO8tHOh/tgCSRNe6WwOwmIF6urZ5uSB8l9aAf5q7iRAIgA4aONKBZhpP5RuOuhJP2dP+2UIRioEJcfU4/m4gZdYo=")
        .send()
        .await?;

    println!("{:#?}", response.text().await?);
    Ok(())
}
