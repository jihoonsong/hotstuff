use hotstuff_mempool::{MempoolError, MempoolTransaction, TransactionPool};
use std::future::Future;

use crate::{to_transaction, FromRpcError, RpcApiError, RpcError, TransactionRequest};

pub(crate) trait RpcApiTransaction {
    type Pool: TransactionPool<Transaction = MempoolTransaction, TransactionError = MempoolError>;

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
                .ok_or(RpcError::InvalidTransactionRequest(request))
                .map_err(Self::Error::from_rpc_err)?;

            let transaction_hash = self
                .pool()
                .add_transaction(transaction)
                .await
                .map_err(Self::Error::from_rpc_err)?;

            Ok(transaction_hash)
        }
    }
}
