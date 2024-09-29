use jsonrpsee_types::ErrorObject;
use std::error::Error;
use std::net::SocketAddr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RpcError {
    #[error("Failed to merge RPC endpoint {0}: {1}")]
    Merge(String, jsonrpsee::core::RegisterMethodError),

    #[error("Failed to build or start the server at {0}: {1}")]
    Server(SocketAddr, std::io::Error),
}

#[derive(Debug, Error)]
pub enum HotStuffApiError {}

impl From<HotStuffApiError> for ErrorObject<'static> {
    fn from(error: HotStuffApiError) -> Self {
        match error {}
    }
}

pub trait HotStuffError: Send + Sync {
    type Error: Into<ErrorObject<'static>> + Error + Send + Sync;
}
