mod common;

use common::mock_keypair;
use hotstuff_crypto::verify;

#[test]
fn test_hex_public_key() {
    let keypair = mock_keypair();
    let hex_pk = keypair.hex_public_key();
    assert_eq!(hex::decode(hex_pk).unwrap(), keypair.pk_share.to_bytes());
}

#[test]
fn test_sign() {
    let keypair = mock_keypair();
    let message = b"test message";
    let signature = keypair.sign(message);
    assert!(verify(
        hex::encode(keypair.pk_share.to_bytes()),
        message,
        signature
    ));
}
