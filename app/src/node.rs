use hotstuff_consensus::HotStuff;
use std::net::SocketAddr;
use tracing::info;

pub struct Node {
    address: SocketAddr,
    peers: Vec<SocketAddr>,
}

impl Node {
    pub fn new(address: SocketAddr, peers: Vec<SocketAddr>) -> Self {
        Self { address, peers }
    }

    pub async fn run(self) {
        // Run HotStuff consensus protocol.
        let hotstuff = HotStuff::new(self.address);
        let mut hotstuff_task = tokio::spawn(hotstuff.run());

        match tokio::try_join!(&mut hotstuff_task) {
            Ok(_) => {
                info!("HotStuff consensus protocol completed");
            }
            Err(e) => {
                info!(error=?e, "An error occured while running HotStuff consensus protocol");
                hotstuff_task.abort();
            }
        }
    }
}
