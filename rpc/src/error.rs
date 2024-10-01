use std::error::Error;
use std::net::SocketAddr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RpcError {
    #[error("Failed to merge RPC endpoint {0}: {1}")]
    Merge(String, jsonrpsee::core::RegisterMethodError),

    #[error("Failed to build or start the server at {0}: {1}")]
    Server(SocketAddr, std::io::Error),

    #[error("Invalid transaction request: {0}")]
    InvalidTransactionRequest(String),
}

impl From<RpcError> for jsonrpsee_types::ErrorObject<'static> {
    fn from(error: RpcError) -> Self {
        match error {
            RpcError::Merge(endpoint, err) => jsonrpsee_types::ErrorObject::owned(
                jsonrpsee_types::error::INTERNAL_ERROR_CODE,
                "Failed to merge RPC endpoint",
                Some(format!("endpoint: {}, error: {}", endpoint, err)),
            ),
            RpcError::Server(socket, err) => jsonrpsee_types::ErrorObject::owned(
                jsonrpsee_types::error::INTERNAL_ERROR_CODE,
                "Failed to build or start the server",
                Some(format!("socket: {}, error: {}", socket, err)),
            ),
            RpcError::InvalidTransactionRequest(request) => jsonrpsee_types::ErrorObject::owned(
                jsonrpsee_types::error::PARSE_ERROR_CODE,
                "Invalid transaction request",
                Some(request),
            ),
        }
    }
}

// A helper trait to convert E to RpcError, and then to jsonrpsee_types::ErrorObjectOwned, where RpcError: From<E>.
pub(crate) trait FromRpcError: From<RpcError> {
    fn from_rpc_err<E>(err: E) -> Self
    where
        RpcError: From<E>;
}

impl<T> FromRpcError for T
where
    T: From<RpcError>,
{
    fn from_rpc_err<E>(err: E) -> Self
    where
        RpcError: From<E>,
    {
        T::from(RpcError::from(err))
    }
}

pub(crate) trait RpcApiError: Send + Sync {
    type Error: Into<jsonrpsee_types::ErrorObject<'static>> + FromRpcError + Error + Send + Sync;
}
