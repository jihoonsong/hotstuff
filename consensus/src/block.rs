use hotstuff_crypto::{Hashable, PublicKey, Signature};
use hotstuff_mempool::Transaction;
use hotstuff_primitives::{BlockHash, Encodable, Round};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio_util::bytes::Bytes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub author: PublicKey,
    pub round: Round,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<T> {
    pub payload: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block<T> {
    pub header: Header,
    pub body: Body<T>,
}

impl<T> Encodable for Block<T>
where
    T: Transaction,
{
    fn encode(self) -> Bytes {
        Bytes::from(bincode::serialize(&self).unwrap())
    }
}

impl<T> Hashable for Block<T>
where
    T: Transaction,
{
    type Hash = BlockHash;

    fn hash(&self) -> Self::Hash {
        Sha256::digest(self.clone().encode()).into()
    }
}

impl<T> Block<T>
where
    T: Transaction,
{
    pub fn seal(self) -> SealedBlock<T> {
        let hash = self.hash();

        SealedBlock {
            header: self.header,
            body: self.body,
            hash,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedBlock<T> {
    header: Header,
    body: Body<T>,
    hash: BlockHash,
}

impl<T> Encodable for SealedBlock<T>
where
    T: Transaction,
{
    fn encode(self) -> Bytes {
        Bytes::from(bincode::serialize(&self).unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedBlock<T> {
    pub block: SealedBlock<T>,
    pub signature: Signature,
}
