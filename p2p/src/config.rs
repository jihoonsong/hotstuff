use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub node_addresses: Vec<SocketAddr>,
}

impl Config {
    pub fn new(nodes: Vec<SocketAddr>) -> Self {
        Self {
            node_addresses: nodes,
        }
    }
}
