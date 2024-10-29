use hotstuff_crypto::PublicKey;
use std::net::SocketAddr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Failed to dial {0}: {1}")]
    Dial(SocketAddr, std::io::Error),

    #[error("Failed to accept incoming connection: {0}")]
    AcceptConnection(std::io::Error),

    #[error("Failed to exchange handshake: {0}")]
    Handshake(std::io::Error),

    #[error("Failed to exchange handshake: connection closed")]
    HandshakeClosed,

    #[error("Failed to exchange handshake: timeout")]
    HandshakeTimeout,

    #[error("Failed to receive message from {0}: {1}")]
    ReceiveMessage(PublicKey, std::io::Error),
}
