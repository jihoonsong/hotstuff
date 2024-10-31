use hotstuff_crypto::{PublicKey, Signer};
use hotstuff_mempool::{Transaction, TransactionPoolExt};
use hotstuff_primitives::{Encodable, Round};
use std::sync::Arc;

use crate::{Block, Body, Header, SignedBlock};

pub struct Proposer<T, P>
where
    T: Transaction,
    P: TransactionPoolExt<Transaction = T>,
{
    identity: PublicKey,
    mempool: Arc<P>,
    signer: Signer,
}

impl<T, P> Proposer<T, P>
where
    T: Transaction,
    P: TransactionPoolExt<Transaction = T>,
{
    pub fn new(identity: PublicKey, mempool: Arc<P>, signer: Signer) -> Self {
        Self {
            identity,
            mempool,
            signer,
        }
    }

    pub async fn propose(&self, round: Round) -> SignedBlock<T> {
        // Create a new block.
        let header = Header {
            author: self.identity.clone(),
            round,
        };
        let body = Body {
            payload: self.mempool.pending_transactions().await,
        };
        let block = Block { header, body };

        // Seal the block.
        let sealed_block = block.seal();

        // Sign the sealed block and return it.
        SignedBlock {
            block: sealed_block.clone(),
            signature: self.signer.sign(sealed_block.encode()),
        }
    }
}
