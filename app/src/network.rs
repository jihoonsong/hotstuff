use futures::future::join_all;
use hotstuff_node::{Node, NodeConfig};

pub struct Network {
    configs: Vec<NodeConfig>,
}

impl Network {
    pub fn new(configs: Vec<NodeConfig>) -> Self {
        Self { configs }
    }

    pub async fn run(self) {
        join_all(
            self.configs
                .into_iter()
                .map(|config| tokio::spawn(Node::new(config).run())),
        )
        .await;
    }
}
