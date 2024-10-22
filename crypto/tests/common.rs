use blsttc::SecretKeySet;
use hotstuff_crypto::KeyPair;

pub fn mock_sk_set() -> SecretKeySet {
    let mut rng = rand::thread_rng();
    SecretKeySet::random(1, &mut rng)
}

pub fn mock_keypair() -> KeyPair {
    let sk_set = mock_sk_set();
    let pk_share = sk_set.public_keys().public_key_share(0);
    let sk_share = sk_set.secret_key_share(0);
    KeyPair { pk_share, sk_share }
}
