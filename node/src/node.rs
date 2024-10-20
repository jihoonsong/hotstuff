use hotstuff_consensus::{HotStuff, RoundRobinLeaderElector};
use hotstuff_mempool::{Mempool, MempoolTransaction, Validator};
use hotstuff_p2p::P2PNetwork;
use hotstuff_rpc::RpcServer;
use tracing::info;

use crate::NodeConfig;

pub struct Node {
    _identity: String, // TODO: Use cryptographic public key.
    configs: NodeConfig,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        Self {
            _identity: config.identity.clone(),
            configs: config,
        }
    }

    pub async fn run(self) {
        // Create a HotStuff mempool.
        let validator = Validator::<MempoolTransaction>::default();
        let mempool = Mempool::<MempoolTransaction, Validator<MempoolTransaction>>::new(validator);

        // Create a leader elector.
        let leader_elector = RoundRobinLeaderElector::new(self.configs.committee);

        // Create a HotStuff consensus protocol.
        let mut hotstuff = HotStuff::new(self.configs.hotstuff, mempool, leader_elector);
        let hotstuff_handler = hotstuff.handler();
        let hotstuff_mempool = hotstuff.mempool();

        // Create a RPC server.
        let rpc_server = RpcServer::new(self.configs.rpc, hotstuff_mempool)
            .build()
            .await
            .expect("Failed to build RPC server");

        // Create a P2P network.
        let p2p_network = P2PNetwork::new(self.configs.network, hotstuff_handler.clone());
        let p2p_network_mailbox = p2p_network.mailbox();

        // Configure the HotStuff consensus protocol.
        hotstuff.set_network(p2p_network_mailbox);

        // Run tasks.
        let mut hotstuff_task = tokio::spawn(hotstuff.run());
        let mut rpc_server_task = tokio::spawn(rpc_server.stopped());
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
