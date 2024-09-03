use crate::handlers::HotStuffHandler;

use hotstuff_p2p::Receiver;
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tracing::info;

pub struct HotStuff {
    address: SocketAddr,
}

impl HotStuff {
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
    }

    pub async fn run(self) {
        // Create MPSC channels for internal communication.
        let (sender, mut _receiver) = mpsc::channel(128);

        // Create a receiver communicating using tokio_util::Framed over TCP.
        let receiver = Receiver::new(self.address, HotStuffHandler { sender });
        let mut receiver_task = tokio::spawn(receiver.run());

        match tokio::try_join!(&mut receiver_task) {
            Ok(_) => {
                info!("All tasks completed");
            }
            Err(e) => {
                info!(error=?e, "An error occured while running tasks");
                receiver_task.abort();
            }
        }
    }
}
