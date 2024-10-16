use std::time::Duration;
use tokio::{
    net::TcpStream,
    sync::{mpsc, oneshot},
    time::sleep,
};
use tracing::{debug, info};

use crate::{PeerManagerMessage, DialerConfig, P2PError};

pub struct Dialer {
    tick: Duration,
    coordinator: mpsc::Sender<PeerManagerMessage>,
}

impl Dialer {
    pub fn new(config: DialerConfig, coordinator: mpsc::Sender<PeerManagerMessage>) -> Self {
        Self {
            tick: Duration::from_secs(config.tick),
            coordinator,
        }
    }

    pub async fn run(self) {
        loop {
            let (respond, response) = oneshot::channel();
            self.coordinator
                .send(PeerManagerMessage::DialablePeers { respond })
                .await
                .unwrap();
            let dialable_peers = response.await.unwrap();

            dialable_peers.into_iter().for_each(|peer| {
                let coordinator = self.coordinator.clone();
                tokio::spawn(async move {
                    match TcpStream::connect(peer)
                        .await
                        .map_err(|e| P2PError::Dial(peer, e))
                    {
                        Ok(stream) => {
                            info!("Successfully dialed {peer}");
                            coordinator
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
