mod common;

use common::{mock_keypair, mock_sk_set};
use hotstuff_crypto::{combine_signatures, verify}; // Use the crate with the correct name
use std::collections::HashMap;

#[test]
fn test_verify() {
    let keypair = mock_keypair();

    let msg = b"test message";
    let signature = hex::encode(keypair.sk_share.sign(msg).to_bytes());
    let result = verify(
        hex::encode(keypair.pk_share.to_bytes()),
        &msg.to_vec(),
        signature,
    );
    assert!(result);
}

#[test]
fn test_combine_signatures() {
    let sk_set = mock_sk_set();
    let pk_set = sk_set.public_keys();
    let msg = b"test message";

    let mut signature_shares = HashMap::new();
    for id in 0..2 {
        let sk_share = sk_set.secret_key_share(id);
        let sig_share = hex::encode(sk_share.sign(msg).to_bytes());
        signature_shares.insert(id, sig_share);
    }

    let sig = combine_signatures(hex::encode(pk_set.to_bytes()), signature_shares)
        .expect("not enough shares");
    assert!(pk_set.public_key().verify(&sig, msg));
}
