use async_trait::async_trait;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};

use crate::Transaction;

#[rpc(server, namespace = "transaction")]
pub trait Transaction {
    #[method(name = "send")]
    async fn send_transaction(&self, request: Transaction) -> RpcResult<String>;
}

pub struct TransactionApi {}

impl TransactionApi {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl TransactionServer for TransactionApi {
    async fn send_transaction(&self, request: Transaction) -> RpcResult<String> {
        Ok(format!("send_transaction received {:?}", request))
    }
}
