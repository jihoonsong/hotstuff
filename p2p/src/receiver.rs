use crate::{error::NetworkError, handler::MessageHandler};

use futures::StreamExt;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::{debug, info};

pub struct Receiver<H: MessageHandler> {
    address: SocketAddr,
    handler: H,
}

impl<H: MessageHandler> Receiver<H> {
    pub fn new(address: SocketAddr, handler: H) -> Self {
        Self { address, handler }
    }

    pub async fn run(self) {
        let listener = TcpListener::bind(self.address)
            .await
            .expect("Failed to bind TCP port");
        info!("Start listening on {}", self.address);

        loop {
            let (stream, peer) = match listener.accept().await {
                Ok(value) => value,
                Err(e) => {
                    debug!(error=?e, "Failed to establish connection");
                    continue;
                }
            };
            info!("Successfully established connection with {peer}");
            Self::spawn_handler(stream, peer, self.handler.clone()).await;
        }
    }

    async fn spawn_handler(stream: TcpStream, peer: SocketAddr, handler: H) {
        tokio::spawn(async move {
            let framed = Framed::new(stream, LengthDelimitedCodec::new());
            let (mut writer, mut reader) = framed.split();

            while let Some(frame) = reader.next().await {
                match frame.map_err(|e| NetworkError::ReceiveMessage(peer, e)) {
                    Ok(message) => {
                        info!("Received {:?} from {peer}", message);
                        handler.dispatch(&mut writer, message.freeze()).await;
                    }
                    Err(e) => {
                        debug!(error=?e, "Failed to read frame from {peer}");
                        break;
                    }
                }
            }
        });
    }
}
