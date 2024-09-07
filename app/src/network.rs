use crate::config::NodeConfig;

use futures::future::join_all;
use hotstuff_node::Node;

pub struct Network {
    node_configs: Vec<NodeConfig>,
}

impl Network {
    pub fn new(node_configs: Vec<NodeConfig>) -> Self {
        Self { node_configs }
    }

    pub async fn run(self) {
        let p2p_addresses: Vec<_> = self
            .node_configs
            .iter()
            .map(|node_config| node_config.p2p_address)
            .collect();

        join_all(self.node_configs.into_iter().map(|node_config| {
            let node = Node::new(
                node_config.identity,
                node_config.p2p_address,
                node_config.rpc_address,
                p2p_addresses
                    .clone()
                    .into_iter()
                    .filter(|p2p_address| *p2p_address != node_config.p2p_address)
                    .collect(),
            );
            tokio::spawn(node.run())
        }))
        .await;
    }
}
