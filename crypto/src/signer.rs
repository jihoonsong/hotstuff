use crate::{SecretKey, Signature};

pub struct Signer(SecretKey);

impl Signer {
    pub fn new(secret_key: SecretKey) -> Self {
        Self(secret_key)
    }

    pub fn sign<M: AsRef<[u8]>>(&self, msg: M) -> Signature {
        self.0.sign(msg)
    }
}
