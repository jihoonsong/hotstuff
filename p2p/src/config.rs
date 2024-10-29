use base64::prelude::{Engine, BASE64_STANDARD};
use hotstuff_crypto::PublicKey;
use serde::Deserialize;
use std::{collections::HashMap, net::SocketAddr};

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub peer_manager: PeerManagerConfig,
    pub dialer: DialerConfig,
    pub listener: ListenerConfig,
    pub mailbox_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct PeerManagerConfig {
    pub min_peers: u16,
    pub max_peers: u16,
    pub mailbox_size: usize,
    pub peers: HashMap<String, SocketAddr>,
    pub handshake_timeout: u64,
}

impl PeerManagerConfig {
    pub(crate) fn peers(&self) -> HashMap<PublicKey, SocketAddr> {
        self.peers
            .iter()
            .map(|(k, v)| {
                (
                    PublicKey::new(BASE64_STANDARD.decode(k).unwrap().try_into().unwrap()),
                    *v,
                )
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct DialerConfig {
    pub interval: u64,
}

#[derive(Debug, Deserialize)]
pub struct ListenerConfig {
    pub address: SocketAddr,
}
