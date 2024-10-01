use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TransactionRequest {
    HotStuff(HotStuffTransactionRequest),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HotStuffTransactionRequest {
    pub nonce: u128,
    pub data: String,
}
