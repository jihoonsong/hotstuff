use crate::Config;

use futures::future::join_all;
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};
use tokio_util::bytes::Bytes;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::{debug, info};

pub struct Client {
    node_addresses: Vec<SocketAddr>,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self {
            node_addresses: config.node_addresses,
        }
    }

    pub async fn run(self) {
        self.connect().await;
    }

    async fn connect(&self) {
        join_all(self.node_addresses.iter().cloned().map(|address| {
            tokio::spawn(async move {
                debug!("Connecting to node at {address}");
                let stream = loop {
                    match TcpStream::connect(address).await {
                        Ok(stream) => break stream,
                        Err(e) => {
                            debug!("Failed to connect to node at {address}: {e}. Retrying...");
                            sleep(Duration::from_millis(100)).await;
                        }
                    }
                };
                info!("Successfully connected to node at {address}");

                // Example of exchanging messages between client and node.
                let mut framed = Framed::new(stream, LengthDelimitedCodec::new());
                framed.send(Bytes::from("Hello from client")).await.unwrap();
                let reply = framed.next().await.unwrap();
                println!("reply: {:?}", reply);
            })
        }))
        .await;
    }
}
