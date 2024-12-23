use parking_lot::RwLock;
use std::{collections::HashSet, future::Future, hash::Hash};

use crate::{MempoolError, Transaction, TransactionValidationResult, TransactionValidator};

pub struct Mempool<T, V>
where
    T: Transaction,
    V: TransactionValidator<Transaction = T>,
{
    validator: V,
    transactions: RwLock<Vec<T>>,
}

impl<T, V> Mempool<T, V>
where
    T: Transaction,
    V: TransactionValidator<Transaction = T>,
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

    type TransactionPoolError;

    fn add_transaction(
        &self,
        transaction: Self::Transaction,
    ) -> impl Future<Output = Result<String, Self::TransactionPoolError>> + Send;
}

pub trait TransactionPoolExt: TransactionPool {
    fn pending_transactions(&self) -> impl Future<Output = Vec<Self::Transaction>> + Send;

    fn on_block_commit(
        &self,
        committed_transactions: Vec<Self::Transaction>,
    ) -> impl Future<Output = ()> + Send;
}

impl<T, V> TransactionPool for Mempool<T, V>
where
    T: Transaction,
    V: TransactionValidator<Transaction = T>,
{
    type Transaction = T;

    type TransactionPoolError = MempoolError;

    async fn add_transaction(
        &self,
        transaction: Self::Transaction,
    ) -> Result<String, Self::TransactionPoolError> {
        match self.validator.validate(transaction).await {
            TransactionValidationResult::Valid(transaction) => {
                let hash = transaction.hash();
                self.transactions.write().push(transaction);
                Ok(hash)
            }
            TransactionValidationResult::Invalid(_, error) => {
                Err(Self::TransactionPoolError::InvalidTransaction(error))
            }
        }
    }
}

impl<T, V> TransactionPoolExt for Mempool<T, V>
where
    T: Transaction + Eq + Hash,
    V: TransactionValidator<Transaction = T>,
{
    async fn pending_transactions(&self) -> Vec<Self::Transaction> {
        self.transactions.read().clone()
    }

    async fn on_block_commit(&self, committed_transactions: Vec<Self::Transaction>) {
        let committed_transactions: HashSet<_> = committed_transactions.into_iter().collect();
        self.transactions
            .write()
            .retain(|tx| !committed_transactions.contains(tx));
    }
}
