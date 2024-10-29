use base64::prelude::{Engine, BASE64_STANDARD};
use blsttc::{PublicKeyShare, SecretKeyShare, PK_SIZE, SIG_SIZE, SK_SIZE};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::signature::Signature;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PublicKey(PublicKeyShare);

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", BASE64_STANDARD.encode(self.to_bytes()))
    }
}

impl PublicKey {
    pub fn new(raw_bytes: [u8; PK_SIZE]) -> Self {
        Self(PublicKeyShare::from_bytes(raw_bytes).unwrap())
    }

    pub fn to_bytes(&self) -> [u8; PK_SIZE] {
        self.0.to_bytes()
    }

    pub fn verify<M: AsRef<[u8]>>(&self, signature: &Signature, msg: M) -> bool {
        self.0.verify(&signature.0, msg)
    }
}

#[derive(Debug, Clone)]
pub struct SecretKey(SecretKeyShare);

impl SecretKey {
    pub fn new(raw_bytes: [u8; SK_SIZE]) -> Self {
        Self(SecretKeyShare::from_bytes(raw_bytes).unwrap())
    }

    pub fn to_bytes(&self) -> [u8; SK_SIZE] {
        self.0.to_bytes()
    }

    pub fn derive_public_key(&self) -> PublicKey {
        let pk_share = self.0.public_key_share();
        PublicKey::new(pk_share.to_bytes())
    }

    pub fn sign(&self, msg: Vec<u8>) -> [u8; SIG_SIZE] {
        self.0.sign(msg).to_bytes()
    }
}

// KeyPair is held by each node.
pub struct KeyPair {
    pub pk: PublicKey,
    pub sk: SecretKey,
}

impl KeyPair {
    pub fn from_sk(sk: SecretKey) -> Self {
        let pk = sk.derive_public_key();
        Self { pk, sk }
    }

    pub fn sign(&self, msg: Vec<u8>) -> Signature {
        let signature = self.sk.sign(msg);
        Signature::new(signature)
    }
}
