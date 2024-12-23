use blsttc::SecretKeySet;
use hotstuff_crypto::{Aggregator, PublicKey, SecretKey, Signature};

// KeyPair is held by each node.
pub struct KeyPair {
    #[allow(dead_code)]
    pub pk: PublicKey,
    pub sk: SecretKey,
}

impl KeyPair {
    pub fn sign<M: AsRef<[u8]>>(&self, msg: M) -> Signature {
        self.sk.sign(msg)
    }
}

pub fn generate_random_keypairs(threshold: usize, n: usize) -> (Aggregator, Vec<KeyPair>) {
    let mut rng = rand::thread_rng();
    let master_sk = SecretKeySet::random(threshold, &mut rng);
    let master_pk = master_sk.public_keys();

    let keypairs: Vec<KeyPair> = (0..n)
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
