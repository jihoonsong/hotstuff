use std::net::SocketAddr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RPCError {
    #[error("Failed to merge RPC endpoint {0}: {1}")]
    Merge(String, jsonrpsee::core::RegisterMethodError),

    #[error("Failed to build or start the server: {0}")]
    Server(SocketAddr, std::io::Error),
}
