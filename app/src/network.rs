use crate::node::Node;

use futures::future::join_all;
use hotstuff_p2p::Config;
use std::net::SocketAddr;

pub struct Network {
    node_addresses: Vec<SocketAddr>,
}

impl Network {
    pub fn new(config: Config) -> Self {
        Self {
            node_addresses: config.node_addresses,
        }
    }

    pub async fn run(self) {
        join_all(self.node_addresses.iter().cloned().map(|address| {
            let node: Node = Node::new(
                address,
                self.node_addresses
                    .clone()
                    .into_iter()
                    .filter(|peer| *peer != address)
                    .collect(),
            );
            tokio::spawn(node.run())
        }))
        .await;
    }
}
