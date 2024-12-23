use std::{future::Future, marker::PhantomData};

use crate::{Transaction, TransactionError, TransactionKind};

pub enum TransactionValidationResult<T: Transaction> {
    Valid(T),
    Invalid(T, TransactionError),
}

pub struct Validator<T>
where
    T: Transaction,
{
    _phantom: PhantomData<T>,
}

impl<T> Validator<T>
where
    T: Transaction,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for Validator<T>
where
    T: Transaction,
{
    fn default() -> Self {
        Self::new()
    }
}

pub trait TransactionValidator: Send + Sync {
    type Transaction: Transaction;

    fn validate(
        &self,
        transaction: Self::Transaction,
    ) -> impl Future<Output = TransactionValidationResult<Self::Transaction>> + Send;
}

impl<T> TransactionValidator for Validator<T>
where
    T: Transaction,
{
    type Transaction = T;

    async fn validate(
        &self,
        transaction: Self::Transaction,
    ) -> TransactionValidationResult<Self::Transaction> {
        match transaction.kind() {
            TransactionKind::Mempool => TransactionValidationResult::Valid(transaction),
            kind => {
                TransactionValidationResult::Invalid(transaction, TransactionError::BadKind(kind))
            }
        }
    }
}
