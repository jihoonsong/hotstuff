use hotstuff_consensus::HotStuff;
use hotstuff_rpc::RpcServer;
use std::net::SocketAddr;
use tracing::info;

pub struct Node {
    id: String, // TODO: Use cryptographic public key.
    p2p_address: SocketAddr,
    rpc_address: SocketAddr,
    peers: Vec<SocketAddr>,
}

impl Node {
    pub fn new(
        id: String,
        p2p_address: SocketAddr,
        rpc_address: SocketAddr,
        peers: Vec<SocketAddr>,
    ) -> Self {
        Self {
            id,
            p2p_address,
            rpc_address,
            peers,
        }
    }

    pub async fn run(self) {
        // Run RPC server.
        let rpc_server = RpcServer::new(self.rpc_address)
            .build()
            .await
            .expect("Failed to build RPC server");
        let mut rpc_server_task = tokio::spawn(rpc_server.stopped());

        // Run HotStuff consensus protocol.
        let hotstuff = HotStuff::new(self.p2p_address);
        let mut hotstuff_task = tokio::spawn(hotstuff.run());

        match tokio::try_join!(&mut rpc_server_task, &mut hotstuff_task) {
            Ok(_) => {
                info!("Node tasks completed");
            }
            Err(e) => {
                info!(error=?e, "An error occured while running HotStuff consensus protocol");
                rpc_server_task.abort();
                hotstuff_task.abort();
            }
        }
    }
}
