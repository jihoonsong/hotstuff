use std::fmt;

use blsttc::{SignatureShare, SIG_SIZE};

use crate::keypair::PublicKey;
pub struct Signature(pub SignatureShare);

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.encode_hex())
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.encode_hex())
    }
}

impl Clone for Signature {
    fn clone(&self) -> Self {
        Self(SignatureShare::from_bytes(self.0.to_bytes()).unwrap())
    }
}

impl Signature {
    pub fn new(raw_bytes: [u8; SIG_SIZE]) -> Self {
        Self(SignatureShare::from_bytes(raw_bytes).unwrap())
    }

    pub fn encode_hex(&self) -> String {
        hex::encode(self.0.to_bytes())
    }

    pub fn to_bytes(&self) -> [u8; SIG_SIZE] {
        self.0.to_bytes()
    }

    pub fn verify(&self, msg: Vec<u8>, author: PublicKey) -> bool {
        author.verify(self.0.to_bytes(), msg)
    }
}
