use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TransactionRequest {
    HotStuff(HotStuffTransactionRequest),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HotStuffTransactionRequest {
    pub nonce: u128,
    pub data: String,
}

impl fmt::Display for TransactionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
