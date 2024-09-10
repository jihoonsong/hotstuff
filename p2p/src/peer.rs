use futures::{stream::SplitStream, StreamExt};
use std::net::SocketAddr;
use tokio::{net::TcpStream, sync::mpsc};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::{debug, info};

use crate::{HotStuffMessage, P2PError};

pub struct Peer {
    identity: SocketAddr, // TODO: Use cryptographic public key.
    reader: Reader,
    hotstuff: mpsc::Sender<HotStuffMessage>,
}

type Reader = SplitStream<Framed<TcpStream, LengthDelimitedCodec>>;

impl Peer {
    pub fn new(
        identity: SocketAddr,
        reader: Reader,
        hotstuff: mpsc::Sender<HotStuffMessage>,
    ) -> Self {
        Self {
            identity,
            reader,
            hotstuff,
        }
    }

    pub async fn run(mut self) {
        while let Some(frame) = self.reader.next().await {
            match frame.map_err(|e| P2PError::ReceiveMessage(self.identity, e)) {
                Ok(message) => {
                    info!("Received message from {}: {:?}", self.identity, message);
                    self.hotstuff
                        .send(HotStuffMessage::Dummy {
                            data: message.freeze(),
                        })
                        .await
                        .unwrap();
                }
                Err(e) => {
                    debug!(error=?e);
                    break;
                }
            }
        }
    }
}
