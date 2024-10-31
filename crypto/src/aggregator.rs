use blsttc::{Error, PublicKeySet};
use std::collections::HashMap;

use crate::public_key::PublicKey;
use crate::signature::Signature;
use crate::ValidatorIndex;

pub struct Aggregator(PublicKeySet);

impl Aggregator {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(PublicKeySet::from_bytes(bytes).unwrap())
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey::new(self.0.public_key().to_bytes())
    }

    pub fn aggregate(
        &self,
        signatures: HashMap<ValidatorIndex, Signature>,
    ) -> Result<Signature, Error> {
        Ok(Signature::new(
            self.0
                .combine_signatures(signatures.iter().map(|(&id, sig)| (id, &sig.0)))?
                .to_bytes(),
        ))
    }
}
