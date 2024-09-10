use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub nodes: Vec<NodeConfig>,
}

#[derive(Debug, Deserialize)]
pub struct NodeConfig {
    pub identity: String,
    pub p2p_address: SocketAddr,
    pub rpc_address: SocketAddr,
    pub peer_addresses: Option<Vec<SocketAddr>>,
}

}
