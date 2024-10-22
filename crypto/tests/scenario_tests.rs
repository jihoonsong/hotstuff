use hotstuff_crypto::{combine_signatures, verify, KeyManager, KeyPair};
use std::collections::HashMap; // Import the utility function

#[test]
fn test_complete_scenario() {
    // Step 1: KeyManager generates keys for a threshold signature system (2 of 3)
    let threshold = 1;
    let n = 3;
    let keymanager = KeyManager::new(threshold, n);

    // Step 2: Derive KeyPair for multiple nodes
    let nodes: Vec<KeyPair> = (0..n).map(|id| keymanager.derive_new_share(id)).collect();
    for (id, node) in nodes.iter().enumerate() {
        println!(
            "Node {} public key: {:?}",
            id,
            hex::encode(node.pk_share.to_bytes())
        );
    }

    // Step 3: Node 0 signs a message, but it doesn't have enough shares
    let mut signature_shares = HashMap::new();
    let msg = b"test message";

    let node_0 = &nodes[0];
    let sig_share_0 = hex::encode(node_0.sk_share.sign(msg).to_bytes());
    signature_shares.insert(0, sig_share_0);

    let result = combine_signatures(
        hex::encode(keymanager.pk_set.to_bytes()),
        signature_shares.clone(),
    );
    assert!(result.is_err());

    // Step 4: Node 0 and Node 1 sign the message
    let node_1 = &nodes[1];
    let sig_share_1 = hex::encode(node_1.sk_share.sign(msg).to_bytes());
    signature_shares.insert(1, sig_share_1);

    let sig = combine_signatures(
        hex::encode(keymanager.pk_set.to_bytes()),
        signature_shares.clone(),
    )
    .expect("not enough shares");
    println!("Combined signature: {:?}", hex::encode(sig.to_bytes()));

    // Step 5: Verify the combined signature
    let result = verify(
        hex::encode(keymanager.pk_set.public_key().to_bytes()),
        &msg.to_vec(),
        hex::encode(sig.to_bytes()),
    );
    assert!(result);

    // Step 6: Node 2 signs the message, and the signature is verified
    let node_2 = &nodes[2];
    let sig_share_2 = hex::encode(node_2.sk_share.sign(msg).to_bytes());
    signature_shares.insert(2, sig_share_2);

    let sig = combine_signatures(
        hex::encode(keymanager.pk_set.to_bytes()),
        signature_shares.clone(),
    )
    .expect("not enough shares");

    let result = verify(
        hex::encode(keymanager.pk_set.public_key().to_bytes()),
        &msg.to_vec(),
        hex::encode(sig.to_bytes()),
    );
    assert!(result);
}
