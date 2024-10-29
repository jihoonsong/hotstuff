use tokio::{
    net::TcpStream,
    sync::{mpsc, oneshot},
    time::{sleep, Duration},
};
use tracing::{debug, info};

use crate::{DialerConfig, NetworkError, PeerManagerMessage};

pub struct Dialer {
    interval: Duration,
    to_peer_manager: mpsc::Sender<PeerManagerMessage>,
}

impl Dialer {
    pub fn new(config: DialerConfig, to_peer_manager: mpsc::Sender<PeerManagerMessage>) -> Self {
        Self {
            interval: Duration::from_millis(config.interval),
            to_peer_manager,
        }
    }

    pub async fn run(self) {
        loop {
            let (reply, response) = oneshot::channel();
            self.to_peer_manager
                .send(PeerManagerMessage::DialablePeers { reply })
                .await
                .unwrap();
            let dialable_peers = response.await.unwrap();

            dialable_peers.into_iter().for_each(|address| {
                let to_peer_manager = self.to_peer_manager.clone();
                tokio::spawn(async move {
                    match TcpStream::connect(address)
                        .await
                        .map_err(|e| NetworkError::Dial(address, e))
                    {
                        Ok(stream) => {
                            info!("Successfully dialed {address}");
                            to_peer_manager
                                .send(PeerManagerMessage::NewPeer { address, stream })
                                .await
                                .unwrap();
                        }
                        Err(e) => {
                            debug!("Failed to dial {address}: {e}");
                        }
                    }
                });
            });

            sleep(self.interval).await;
        }
    }
}
