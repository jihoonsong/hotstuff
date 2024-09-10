use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
pub struct RpcConfig {
    pub address: SocketAddr,
}
