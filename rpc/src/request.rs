use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionRequest {
    pub nonce: u128,
    pub data: String,
}
