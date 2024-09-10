use hotstuff_consensus::{HotStuff, HotStuffConfig};
use hotstuff_p2p::{Network, NetworkConfig};
use hotstuff_rpc::{RpcConfig, RpcServer};
use tracing::info;

use crate::NodeConfig;

pub struct Node {
    identity: String, // TODO: Use cryptographic public key.
    hotstuff: HotStuffConfig,
    rpc: RpcConfig,
    network: NetworkConfig,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        Self {
            identity: config.identity,
            hotstuff: config.hotstuff,
            rpc: config.rpc,
            network: config.network,
        }
    }

    pub async fn run(self) {
        // Run HotStuff consensus protocol.
        let hotstuff = HotStuff::new(self.hotstuff);
        let hotstuff_mailbox = hotstuff.mailbox();
        let mut hotstuff_task = tokio::spawn(hotstuff.run());

        // Run RPC server.
        let rpc_server = RpcServer::new(self.rpc)
            .build()
            .await
            .expect("Failed to build RPC server");
        let mut rpc_server_task = tokio::spawn(rpc_server.stopped());

        // Run P2P network.
        let p2p_network = Network::new(self.network, hotstuff_mailbox.clone());
        let mut p2p_network_task = tokio::spawn(p2p_network.run());

        match tokio::try_join!(
            &mut hotstuff_task,
            &mut rpc_server_task,
            &mut p2p_network_task,
        ) {
            Ok(_) => {
                info!("Node tasks completed");
            }
            Err(e) => {
                info!(error=?e, "An error occured while running HotStuff consensus protocol");
                hotstuff_task.abort();
                rpc_server_task.abort();
                p2p_network_task.abort();
            }
        }
    }
}
