use std::net::SocketAddr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum P2PError {
    #[error("Failed to dial {0}: {1}")]
    Dial(SocketAddr, std::io::Error),

    #[error("Failed to accept incoming connection: {0}")]
    AcceptConnection(std::io::Error),

    #[error("Failed to receive message from {0}: {1}")]
    ReceiveMessage(SocketAddr, std::io::Error),
}
