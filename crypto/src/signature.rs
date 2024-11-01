use blsttc::{SignatureShare, SIG_SIZE};
use serde::{Deserialize, Serialize};

use crate::PublicKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature(pub(crate) SignatureShare);

impl Signature {
    pub fn new(bytes: [u8; SIG_SIZE]) -> Self {
        Self(SignatureShare::from_bytes(bytes).unwrap())
    }

    pub fn verify<M: AsRef<[u8]>>(&self, author: &PublicKey, msg: M) -> bool {
        author.0.verify(&self.0, msg)
    }
}
