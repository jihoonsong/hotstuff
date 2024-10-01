use parking_lot::RwLock;
use std::future::Future;

use crate::{error::MempoolError, Transaction};

pub struct Mempool<T>
where
    T: Transaction + Send + Sync + Clone,
{
    transactions: RwLock<Vec<T>>,
}

impl<T> Mempool<T>
where
    T: Transaction + Send + Sync + Clone,
{
    pub fn new() -> Self {
        Self {
            transactions: RwLock::new(Vec::new()),
        }
    }
}

pub trait TransactionPool {
    type Transaction;

    type TransactionError;

    fn add_transaction(
        &self,
        transaction: Self::Transaction,
    ) -> impl Future<Output = Result<String, Self::TransactionError>> + Send;
}

impl<T> TransactionPool for Mempool<T>
where
    T: Transaction + Send + Sync + Clone,
{
    type Transaction = T;

    type TransactionError = MempoolError;

    fn add_transaction(
        &self,
        transaction: Self::Transaction,
    ) -> impl Future<Output = Result<String, Self::TransactionError>> + Send {
        async move {
            let hash = transaction.hash();
            self.transactions.write().push(transaction);
            Ok(hash)
        }
    }
}
