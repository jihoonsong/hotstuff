use blsttc::{PublicKeySet, SecretKeySet};

use crate::keypair::KeyPair;

// KeyManager is a single trusted dealer that generates keys for a group of users.
// It is reponsible for holding public key set and secret key set.
pub struct KeyManager {
    n: usize,
    pk_set: PublicKeySet,
    sk_set: SecretKeySet,
}

impl KeyManager {
    pub fn new(threshold: usize, n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let sk_set = SecretKeySet::random(threshold, &mut rng);
        let pk_set = sk_set.public_keys();

        Self { n, pk_set, sk_set }
    }

    pub fn derive_new_share(&self, id: usize) -> KeyPair {
        let sk_share = self.sk_set.secret_key_share(id);
        let pk_share = self.pk_set.public_key_share(id);

        KeyPair { pk_share, sk_share }
    }
}
