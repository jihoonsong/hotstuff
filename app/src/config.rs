use hotstuff_node::NodeConfig;
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub client: ClientConfig,
    pub nodes: Vec<NodeConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ClientConfig {
    pub nodes: Vec<SocketAddr>,
}
