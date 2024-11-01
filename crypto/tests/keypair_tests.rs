mod utils;
use crate::utils::generate_random_keypairs;

#[test]
fn test_sign_and_verify() {
    let (_, keypairs) = generate_random_keypairs(1, 1);
    let keypair = &keypairs[0];
    let message = b"test message";
    let signature = keypair.sign(message.to_vec());
    assert!(signature.verify(&keypair.pk, message.to_vec()));
}
