use hotstuff_mempool::Transaction;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::Round;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block<T> {
    author: SocketAddr, // TODO: Replace with cryptographic public key.
    round: Round,
    payload: Vec<T>,
}

impl<T> Block<T>
where
    T: Transaction,
{
    pub fn new(author: SocketAddr, round: Round, payload: Vec<T>) -> Self {
        Self {
            author,
            round,
            payload,
        }
    }
}
