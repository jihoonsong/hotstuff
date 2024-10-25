use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum TransactionKind {
    Mempool,
    Placeholder, // To be replaced.
}

impl fmt::Display for TransactionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MempoolTransaction {
    pub nonce: u128,
    pub data: String,
    pub kind: TransactionKind,
}

pub trait Transaction {
    fn hash(&self) -> String;

    fn nonce(&self) -> u128;

    fn data(&self) -> String;

    fn kind(&self) -> TransactionKind;
}

impl Transaction for MempoolTransaction {
    fn hash(&self) -> String {
        format!("0x{}", self.nonce) // TODO: Return cryptographic hash
    }

    fn nonce(&self) -> u128 {
        self.nonce
    }

    fn data(&self) -> String {
        self.data.clone()
    }

    fn kind(&self) -> TransactionKind {
        self.kind.clone()
    }
}
