use std::net::SocketAddr;
use tokio::{net::TcpListener, sync::mpsc};
use tracing::{debug, info};

use crate::{ListenerConfig, NetworkError, PeerManagerMessage};

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
            match listener
                .accept()
                .await
                .map_err(NetworkError::AcceptConnection)
            {
                Ok((stream, address)) => {
                    info!("Successfully accepted incoming connection from {address}");
                    self.to_peer_manager
                        .send(PeerManagerMessage::NewPeer { address, stream })
                        .await
                        .unwrap();
                }
                Err(e) => {
                    debug!("Failed to accept incoming connection: {e}");
                    continue;
                }
            }
        }
    }
}
