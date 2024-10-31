use hotstuff_consensus::{Committee, HotStuff, HotStuffConfig, RoundRobinLeaderElector};
use hotstuff_crypto::{PublicKey, SecretKey, Signer, ValidatorIndex};
use hotstuff_mempool::{Mempool, MempoolTransaction, Validator};
use hotstuff_p2p::{NetworkConfig, P2PNetwork};
use hotstuff_rpc::{RpcConfig, RpcServer};
use std::collections::HashMap;
use tracing::info;

use crate::NodeConfig;

pub struct Node {
    public_key: PublicKey,
    secret_key: SecretKey,
    committee: HashMap<PublicKey, ValidatorIndex>,
    hotstuff_config: HotStuffConfig,
    rpc_config: RpcConfig,
    network_config: NetworkConfig,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        Self {
            public_key: config.public_key(),
            secret_key: config.secret_key(),
            committee: config.committee(),
            hotstuff_config: config.hotstuff,
            rpc_config: config.rpc,
            network_config: config.network,
        }
    }

    pub async fn run(self) {
        // Create a HotStuff mempool.
        let validator = Validator::<MempoolTransaction>::default();
        let mempool = Mempool::<MempoolTransaction, Validator<MempoolTransaction>>::new(validator);

        // Create the committee.
        let committee = Committee::new(
            self.committee.clone(),
            RoundRobinLeaderElector::new(self.committee.keys().cloned().collect()),
        );

        // Create a signer.
        let signer = Signer::new(self.secret_key);

        // Create a HotStuff consensus protocol.
        let mut hotstuff = HotStuff::new(
            self.hotstuff_config,
            mempool,
            committee,
            self.public_key.clone(),
            signer,
        );
        let hotstuff_handler = hotstuff.handler();
        let hotstuff_mempool = hotstuff.mempool();

        // Create a RPC server.
        let rpc_server = RpcServer::new(self.rpc_config, hotstuff_mempool)
            .build()
            .await
            .expect("Failed to build RPC server");

        // Create a P2P network.
        let p2p_network = P2PNetwork::new(
            self.network_config,
            hotstuff_handler.clone(),
            self.public_key.clone(),
        );
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
