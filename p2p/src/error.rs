use std::net::SocketAddr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Failed to receive message from {0}: {1}")]
    ReceiveMessage(SocketAddr, std::io::Error),
}
