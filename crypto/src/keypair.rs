use blsttc::{PublicKeyShare, SecretKeyShare};
use hex;

// KeyPair is a public key share and a secret key share that each node holds.
pub struct KeyPair {
    pub pk_share: PublicKeyShare,
    pub sk_share: SecretKeyShare,
}

impl KeyPair {
    pub fn hex_public_key(&self) -> String {
        hex::encode(self.pk_share.to_bytes())
    }

    pub fn sign(&self, msg: &[u8]) -> String {
        hex::encode(self.sk_share.sign(msg).to_bytes())
    }
}
