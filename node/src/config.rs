use base64::prelude::{Engine, BASE64_STANDARD};
use hotstuff_consensus::HotStuffConfig;
use hotstuff_crypto::{PublicKey, SecretKey, ValidatorIndex};
use hotstuff_p2p::NetworkConfig;
use hotstuff_rpc::RpcConfig;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct NodeConfig {
    pub public_key: String,
    pub secret_key: String,
    pub committee: Vec<String>,
    pub hotstuff: HotStuffConfig,
    pub rpc: RpcConfig,
    pub network: NetworkConfig,
}

impl NodeConfig {
    pub(crate) fn public_key(&self) -> PublicKey {
        PublicKey::new(
            BASE64_STANDARD
                .decode(&self.public_key)
                .unwrap()
                .try_into()
                .unwrap(),
        )
    }

    pub(crate) fn secret_key(&self) -> SecretKey {
        SecretKey::new(
            BASE64_STANDARD
                .decode(&self.secret_key)
                .unwrap()
                .try_into()
                .unwrap(),
        )
    }

    pub(crate) fn committee(&self) -> HashMap<PublicKey, ValidatorIndex> {
        self.committee
            .iter()
            .map(|c| PublicKey::new(BASE64_STANDARD.decode(c).unwrap().try_into().unwrap()))
            .enumerate()
            .map(|(i, pk)| (pk, i))
            .collect()
    }
}
