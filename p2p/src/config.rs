use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub peer_manager: PeerManagerConfig,
    pub dialer: DialerConfig,
    pub listener: ListenerConfig,
}

#[derive(Debug, Deserialize)]
pub struct PeerManagerConfig {
    pub max_peers: u16,
    pub mailbox_size: usize,
    pub peers: Option<Vec<SocketAddr>>,
}

#[derive(Debug, Deserialize)]
pub struct DialerConfig {
    pub tick: u64,
}

#[derive(Debug, Deserialize)]
pub struct ListenerConfig {
    pub address: SocketAddr,
}
