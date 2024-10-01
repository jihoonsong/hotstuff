use hotstuff_consensus::Transaction;

#[derive(Clone)]
pub struct MempoolTransaction {
    pub nonce: u128,
    pub data: String,
}

impl Transaction for MempoolTransaction {
    fn hash(&self) -> String {
        format!("0x{}", self.nonce) // TODO: Return cryptographic hash
    }
}
