use hotstuff_crypto::PublicKey;
use hotstuff_mempool::Transaction;
use serde::{Deserialize, Serialize};

use crate::Round;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block<T> {
    author: PublicKey,
    round: Round,
    payload: Vec<T>,
}

impl<T> Block<T>
where
    T: Transaction,
{
    pub fn new(author: PublicKey, round: Round, payload: Vec<T>) -> Self {
        Self {
            author,
            round,
            payload,
        }
    }
}
