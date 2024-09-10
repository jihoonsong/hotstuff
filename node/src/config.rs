use hotstuff_consensus::HotStuffConfig;
use hotstuff_p2p::NetworkConfig;
use hotstuff_rpc::RpcConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NodeConfig {
    pub identity: String,
    pub hotstuff: HotStuffConfig,
    pub rpc: RpcConfig,
    pub network: NetworkConfig,
}
