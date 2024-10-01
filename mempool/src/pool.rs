use parking_lot::RwLock;
use std::future::Future;

use crate::{error::MempoolError, Transaction, TransactionValidationResult, TransactionValidator};

pub struct Mempool<T, V>
where
    T: Transaction + Send + Sync,
    V: TransactionValidator<Transaction = T> + Send + Sync,
{
    validator: V,
    transactions: RwLock<Vec<T>>,
}

impl<T, V> Mempool<T, V>
where
    T: Transaction + Send + Sync,
    V: TransactionValidator<Transaction = T> + Send + Sync,
{
    pub fn new(validator: V) -> Self {
        Self {
            validator,
            transactions: RwLock::new(Vec::new()),
        }
    }
}

pub trait TransactionPool {
    type Transaction: Transaction;

    type TransactionError;

    fn add_transaction(
        &self,
        transaction: Self::Transaction,
    ) -> impl Future<Output = Result<String, Self::TransactionError>> + Send;
}

impl<T, V> TransactionPool for Mempool<T, V>
where
    T: Transaction + Send + Sync,
    V: TransactionValidator<Transaction = T> + Send + Sync,
{
    type Transaction = T;

    type TransactionError = MempoolError;

    fn add_transaction(
        &self,
        transaction: Self::Transaction,
    ) -> impl Future<Output = Result<String, Self::TransactionError>> + Send {
        async move {
            match self.validator.validate(transaction).await {
                TransactionValidationResult::Valid(transaction) => {
                    let hash = transaction.hash();
                    self.transactions.write().push(transaction);
                    Ok(hash)
                }
                TransactionValidationResult::Invalid(_, error) => {
                    Err(Self::TransactionError::InvalidTransaction(error))
                }
            }
        }
    }
}
