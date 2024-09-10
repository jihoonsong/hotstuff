use futures::future::join_all;
use hotstuff_node::Node;

use crate::config::NodeConfig;

pub struct Network {
    node_configs: Vec<NodeConfig>,
}

impl Network {
    pub fn new(node_configs: Vec<NodeConfig>) -> Self {
        Self { node_configs }
    }

    pub async fn run(self) {
        join_all(self.node_configs.into_iter().map(|node_config| {
            let node = Node::new(
                node_config.identity,
                node_config.p2p_address,
                node_config.rpc_address,
                node_config.peer_addresses.unwrap_or(vec![]),
            );
            tokio::spawn(node.run())
        }))
        .await;
    }
}
