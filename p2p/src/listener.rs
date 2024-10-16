use std::net::SocketAddr;
use tokio::{net::TcpListener, sync::mpsc};
use tracing::{debug, info};

use crate::{PeerManagerMessage, ListenerConfig, P2PError};

pub struct Listener {
    address: SocketAddr,
    coordinator: mpsc::Sender<PeerManagerMessage>,
}

impl Listener {
    pub fn new(config: ListenerConfig, coordinator: mpsc::Sender<PeerManagerMessage>) -> Self {
        Self {
            address: config.address,
            coordinator,
        }
    }

    pub async fn run(self) {
        let listener = TcpListener::bind(self.address)
            .await
            .expect("Failed to bind TCP port");
        info!("Start listening on {}", self.address);

        loop {
            match listener.accept().await.map_err(P2PError::AcceptConnection) {
                Ok((stream, peer)) => {
                    info!("Successfully accepted incoming connection from {peer}");
                    self.coordinator
                        .send(PeerManagerMessage::NewPeer { peer, stream })
                        .await
                        .unwrap();
                }
                Err(e) => {
                    debug!(error=?e);
                    continue;
                }
            }
        }
    }
}
