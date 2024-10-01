use hotstuff_consensus::TransactionPool;
use hotstuff_mempool::MempoolTransaction;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use std::sync::Arc;
use tracing::info;

use crate::{RpcApiError, RpcApiTransaction, RpcApis, RpcError, TransactionRequest};

pub(crate) struct RpcApi<T>
where
    T: TransactionPool<Transaction = MempoolTransaction> + Send + Sync + 'static,
{
    transaction_pool: Arc<T>,
}

impl<T> RpcApi<T>
where
    T: TransactionPool<Transaction = MempoolTransaction> + Send + Sync + 'static,
{
    pub fn new(transaction_pool: Arc<T>) -> Self {
        Self { transaction_pool }
    }
}
#[rpc(server, namespace = "hotstuff")]
pub(crate) trait RpcApi {
    #[method(name = "sendTransaction")]
    async fn send_transaction(&self, request: TransactionRequest) -> RpcResult<String>;
}

#[async_trait::async_trait]
impl<T> RpcApiServer for T
where
    T: RpcApis,
    jsonrpsee_types::ErrorObject<'static>: From<T::Error>,
{
    async fn send_transaction(&self, request: TransactionRequest) -> RpcResult<String> {
        info!("send_transaction: {:?}", request);
        Ok(RpcApiTransaction::send_transaction(self, request).await?)
    }
}

impl<T> RpcApiError for RpcApi<T>
where
    T: TransactionPool<Transaction = MempoolTransaction> + Send + Sync + 'static,
{
    type Error = RpcError;
}

impl<T> RpcApiTransaction for RpcApi<T>
where
    T: TransactionPool<Transaction = MempoolTransaction> + Send + Sync + 'static,
{
    type Pool = T;

    fn pool(&self) -> &Self::Pool {
        &self.transaction_pool
    }
}
