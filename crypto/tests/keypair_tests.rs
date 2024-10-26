use hotstuff_crypto::generate_random_keypairs;

#[test]
fn test_encode_hex() {
    let (_, keypairs) = generate_random_keypairs(1, 1);
    let keypair = &keypairs[0];
    assert_eq!(
        format!("{}", keypair.pk),
        hex::encode(keypair.pk.to_bytes())
    );
}

#[test]
fn test_sign_and_verify() {
    let (_, keypairs) = generate_random_keypairs(1, 1);
    let keypair = &keypairs[0];
    let message = b"test message";
    let signature = keypair.sign(message.to_vec());
    assert!(keypair.pk.verify(signature.to_bytes(), message.to_vec()));
}
