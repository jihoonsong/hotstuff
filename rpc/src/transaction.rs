use hotstuff_consensus::TransactionPool;
use hotstuff_mempool::MempoolTransaction;
use std::future::Future;

use crate::{to_transaction, FromRpcError, RpcApiError, RpcError, TransactionRequest};

pub(crate) trait RpcApiTransaction {
    type Pool: TransactionPool<Transaction = MempoolTransaction>;

    fn pool(&self) -> &Self::Pool;

    fn send_transaction(
        &self,
        request: TransactionRequest,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send
    where
        Self: RpcApiError,
    {
        async move {
            let transaction = to_transaction(request.clone())
                .ok_or(RpcError::InvalidTransactionRequest(format!(
                    "{:?}",
                    request
                )))
                .map_err(Self::Error::from_rpc_err)?;

            _ = self.pool().add_transaction(transaction).await;

            Ok(String::from(""))
        }
    }
}
