use blsttc::{SignatureShare, SIG_SIZE};

use crate::keypair::PublicKey;

#[derive(Debug, Clone)]
pub struct Signature(pub SignatureShare);

impl Signature {
    pub fn new(raw_bytes: [u8; SIG_SIZE]) -> Self {
        Self(SignatureShare::from_bytes(raw_bytes).unwrap())
    }

    pub fn to_bytes(&self) -> [u8; SIG_SIZE] {
        self.0.to_bytes()
    }

    pub fn verify<M: AsRef<[u8]>>(&self, author: &PublicKey, msg: M) -> bool {
        author.verify(self, msg)
    }
}
