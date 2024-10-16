use std::net::SocketAddr;
use tokio::{net::TcpListener, sync::mpsc};
use tracing::{debug, info};

use crate::{ListenerConfig, P2PError, PeerManagerMessage};

pub struct Listener {
    address: SocketAddr,
    to_peer_manager: mpsc::Sender<PeerManagerMessage>,
}

impl Listener {
    pub fn new(config: ListenerConfig, to_peer_manager: mpsc::Sender<PeerManagerMessage>) -> Self {
        Self {
            address: config.address,
            to_peer_manager,
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
                    self.to_peer_manager
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
