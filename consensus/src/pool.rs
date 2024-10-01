use std::future::Future;

pub trait TransactionPool {
    type Transaction;

    type TransactionError;

    fn add_transaction(
        &self,
        transaction: Self::Transaction,
    ) -> impl Future<Output = Result<String, Self::TransactionError>> + Send;
}
