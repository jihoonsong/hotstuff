use std::fmt;

use blsttc::{
    PublicKeyShare, SecretKeySet, SecretKeyShare, SignatureShare, PK_SIZE, SIG_SIZE, SK_SIZE,
};

use crate::aggregate::Aggregator;
use crate::signature::Signature;

pub struct PublicKey(PublicKeyShare);

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.encode_hex())
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.encode_hex())
    }
}

impl Clone for PublicKey {
    fn clone(&self) -> Self {
        Self(PublicKeyShare::from_bytes(self.0.to_bytes()).unwrap())
    }
}

impl PublicKey {
    pub fn new(raw_bytes: [u8; PK_SIZE]) -> Self {
        Self(PublicKeyShare::from_bytes(raw_bytes).unwrap())
    }

    pub fn encode_hex(&self) -> String {
        hex::encode(self.0.to_bytes())
    }

    pub fn to_bytes(&self) -> [u8; PK_SIZE] {
        self.0.to_bytes()
    }

    pub fn verify<M: AsRef<[u8]>>(&self, signature: [u8; SIG_SIZE], msg: M) -> bool {
        let signature = SignatureShare::from_bytes(signature).unwrap();
        self.0.verify(&signature, msg)
    }
}

pub struct SecretKey(SecretKeyShare);

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.encode_hex())
    }
}

impl fmt::Display for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.encode_hex())
    }
}

impl Clone for SecretKey {
    fn clone(&self) -> Self {
        Self(SecretKeyShare::from_bytes(self.0.to_bytes()).unwrap())
    }
}

impl SecretKey {
    pub fn new(raw_bytes: [u8; SK_SIZE]) -> Self {
        Self(SecretKeyShare::from_bytes(raw_bytes).unwrap())
    }

    pub fn from_hex(hex: &String) -> Self {
        let raw_bytes = hex::decode(hex).unwrap();
        Self::new(raw_bytes.try_into().unwrap())
    }

    pub fn encode_hex(&self) -> String {
        hex::encode(self.0.to_bytes())
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

pub fn generate_random_keypairs(threshold: usize, n: usize) -> (Aggregator, Vec<KeyPair>) {
    let mut rng = rand::thread_rng();
    let master_sk = SecretKeySet::random(threshold, &mut rng);
    let master_pk = master_sk.public_keys();

    let keypairs = (0..n)
        .map(|id| {
            let sk_share = master_sk.secret_key_share(id);
            let pk_share = master_pk.public_key_share(id);
            KeyPair {
                pk: PublicKey::new(pk_share.to_bytes()),
                sk: SecretKey::new(sk_share.to_bytes()),
            }
        })
        .collect();

    (Aggregator::new(master_pk.to_bytes()), keypairs)
}
