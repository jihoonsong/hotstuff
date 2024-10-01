use std::fmt;

#[derive(Clone, Debug)]
pub enum TransactionKind {
    Mempool = 0,
    Placeholder, // To be replaced.
}

impl fmt::Display for TransactionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct MempoolTransaction {
    pub nonce: u128,
    pub data: String,
    pub kind: TransactionKind,
}

pub trait Transaction {
    fn hash(&self) -> String;

    fn kind(&self) -> TransactionKind;
}

impl Transaction for MempoolTransaction {
    fn hash(&self) -> String {
        format!("0x{}", self.nonce) // TODO: Return cryptographic hash
    }

    fn kind(&self) -> TransactionKind {
        self.kind.clone()
    }
}
