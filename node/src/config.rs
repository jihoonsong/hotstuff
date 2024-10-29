use base64::prelude::{Engine, BASE64_STANDARD};
use hotstuff_consensus::HotStuffConfig;
use hotstuff_crypto::PublicKey;
use hotstuff_p2p::NetworkConfig;
use hotstuff_rpc::RpcConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NodeConfig {
    pub identity: String,
    pub committee: Vec<String>,
    pub hotstuff: HotStuffConfig,
    pub rpc: RpcConfig,
    pub network: NetworkConfig,
}

impl NodeConfig {
    pub(crate) fn identity(&self) -> PublicKey {
        PublicKey::new(
            BASE64_STANDARD
                .decode(&self.identity)
                .unwrap()
                .try_into()
                .unwrap(),
        )
    }

    pub(crate) fn committee(&self) -> Vec<PublicKey> {
        self.committee
            .iter()
            .map(|c| PublicKey::new(BASE64_STANDARD.decode(c).unwrap().try_into().unwrap()))
            .collect()
    }
}
