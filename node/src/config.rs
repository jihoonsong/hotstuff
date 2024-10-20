use hotstuff_consensus::HotStuffConfig;
use hotstuff_p2p::NetworkConfig;
use hotstuff_rpc::RpcConfig;
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
pub struct NodeConfig {
    pub identity: String,
    pub committee: Vec<SocketAddr>,
    pub hotstuff: HotStuffConfig,
    pub rpc: RpcConfig,
    pub network: NetworkConfig,
}
