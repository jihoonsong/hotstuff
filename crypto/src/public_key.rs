use base64::prelude::{Engine, BASE64_STANDARD};
use blsttc::{PublicKeyShare, PK_SIZE};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PublicKey(pub(crate) PublicKeyShare);

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", BASE64_STANDARD.encode(self.0.to_bytes()))
    }
}

impl PublicKey {
    pub fn new(bytes: [u8; PK_SIZE]) -> Self {
        Self(PublicKeyShare::from_bytes(bytes).unwrap())
    }
}
