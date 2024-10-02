use hotstuff_mempool::{MempoolTransaction, TransactionKind};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::RpcError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TransactionRequest {
    Mempool(MempoolTransactionRequest),
    Placeholder, // To be replaced.
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MempoolTransactionRequest {
    pub nonce: u128,
    pub data: String,
}

impl fmt::Display for TransactionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<TransactionRequest> for MempoolTransaction {
    type Error = RpcError;

    fn try_from(request: TransactionRequest) -> Result<Self, Self::Error> {
        match request {
            TransactionRequest::Mempool(request) => Ok(MempoolTransaction {
                nonce: request.nonce,
                data: request.data,
                kind: TransactionKind::Mempool,
            }),
            _ => Err(RpcError::InvalidTransactionRequest(request)),
        }
    }
}
