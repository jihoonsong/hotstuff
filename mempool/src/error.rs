use thiserror::Error;

use crate::TransactionKind;

#[derive(Debug, Error)]
pub enum MempoolError {
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(TransactionError),
}

#[derive(Debug, Error)]
pub enum TransactionError {
    #[error("Bad nonce")]
    BadNonce(),
    #[error("Bad data")]
    BadData(),
    #[error("Bad kind: {0}")]
    BadKind(TransactionKind),
}
