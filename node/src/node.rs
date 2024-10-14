use hotstuff_consensus::HotStuff;
use hotstuff_mempool::{Mempool, MempoolTransaction, Validator};
use hotstuff_p2p::Network;
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
        // Create transaction validator.
        let validator = Validator::<MempoolTransaction>::default();

        // Create HotStuff mempool.
        let mempool = Mempool::<MempoolTransaction, Validator<MempoolTransaction>>::new(validator);

        // Run HotStuff consensus protocol.
        let hotstuff = HotStuff::new(self.configs.hotstuff, mempool);
        let hotstuff_mailbox = hotstuff.mailbox();
        let hotstuff_mempool = hotstuff.mempool();
        let mut hotstuff_task = tokio::spawn(hotstuff.run());

        // Run RPC server.
        let rpc_server = RpcServer::new(self.configs.rpc, hotstuff_mempool)
            .build()
            .await
            .expect("Failed to build RPC server");
        let mut rpc_server_task = tokio::spawn(rpc_server.stopped());

        // Run P2P network.
        let p2p_network = Network::new(self.configs.network, hotstuff_mailbox.clone());
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
