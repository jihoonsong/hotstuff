use blsttc::{SecretKeyShare, SK_SIZE};

use crate::Signature;

#[derive(Debug, Clone)]
pub struct SecretKey(pub(crate) SecretKeyShare);

impl SecretKey {
    pub fn new(bytes: [u8; SK_SIZE]) -> Self {
        Self(SecretKeyShare::from_bytes(bytes).unwrap())
    }

    pub fn sign<M: AsRef<[u8]>>(&self, msg: M) -> Signature {
        Signature::new(self.0.sign(msg).to_bytes())
    }
}
