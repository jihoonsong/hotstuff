use blsttc::{Error, PublicKeySet, SignatureShare};
use std::collections::HashMap;

use crate::keypair::PublicKey;
use crate::signature::Signature;

pub struct Aggregator(PublicKeySet);

impl Aggregator {
    pub fn new(raw_bytes: Vec<u8>) -> Self {
        Self(PublicKeySet::from_bytes(raw_bytes).unwrap())
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey::new(self.0.public_key().to_bytes())
    }

    pub fn aggregate_signatures(
        &self,
        signatures: HashMap<usize, Signature>,
    ) -> Result<Signature, Error> {
        let signatures: HashMap<usize, SignatureShare> =
            signatures
                .iter()
                .fold(HashMap::new(), |mut acc, (id, signature)| {
                    acc.insert(*id, signature.0.clone());
                    acc
                });
        Ok(Signature::new(
            self.0.combine_signatures(signatures)?.to_bytes(),
        ))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_bytes()
    }
}
