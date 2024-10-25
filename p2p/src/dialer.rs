use std::time::Duration;
use tokio::{
    net::TcpStream,
    sync::{mpsc, oneshot},
    time::sleep,
};
use tracing::{debug, info};

use crate::{DialerConfig, NetworkError, PeerManagerMessage};

pub struct Dialer {
    tick: Duration,
    to_peer_manager: mpsc::Sender<PeerManagerMessage>,
}

impl Dialer {
    pub fn new(config: DialerConfig, to_peer_manager: mpsc::Sender<PeerManagerMessage>) -> Self {
        Self {
            tick: Duration::from_secs(config.tick),
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

            dialable_peers.into_iter().for_each(|peer| {
                let to_peer_manager = self.to_peer_manager.clone();
                tokio::spawn(async move {
                    match TcpStream::connect(peer)
                        .await
                        .map_err(|e| NetworkError::Dial(peer, e))
                    {
                        Ok(stream) => {
                            info!("Successfully dialed {peer}");
                            to_peer_manager
                                .send(PeerManagerMessage::NewPeer { peer, stream })
                                .await
                                .unwrap();
                        }
                        Err(e) => {
                            debug!(error=?e);
                        }
                    }
                });
            });

            sleep(self.tick).await;
        }
    }
}
