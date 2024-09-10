use futures::future::join_all;
use hotstuff_node::{Node, NodeConfig};

pub struct Network {
    nodes: Vec<NodeConfig>,
}

impl Network {
    pub fn new(nodes: Vec<NodeConfig>) -> Self {
        Self { nodes }
    }

    pub async fn run(self) {
        join_all(
            self.nodes
                .into_iter()
                .map(|node| tokio::spawn(Node::new(node).run())),
        )
        .await;
    }
}
