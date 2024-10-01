use hotstuff_mempool::{MempoolError, MempoolTransaction, TransactionPool};
use std::future::Future;

use crate::{FromRpcError, RpcApiError, TransactionRequest};

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
            let transaction = request.try_into().map_err(Self::Error::from_rpc_err)?;
            let transaction_hash = self
                .pool()
                .add_transaction(transaction)
                .await
                .map_err(Self::Error::from_rpc_err)?;
            Ok(transaction_hash)
        }
    }
}
