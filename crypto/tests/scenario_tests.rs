use std::collections::HashMap;

mod utils;
use crate::utils::generate_random_keypairs;

#[test]
fn test_complete_scenario() {
    // Step 1: Initialize new nodes with random keypair system based on a threshold signature system (2 of 3)
    let threshold = 1;
    let n = 3;
    let (aggregator, nodes) = generate_random_keypairs(threshold, n);
    for (id, node) in nodes.iter().enumerate() {
        println!("Node {} public key: {:?}", id, node.pk);
    }

    // Step 2: Node 0 signs a message, but it doesn't have enough shares
    let mut signatures = HashMap::new();
    let msg = b"test message";

    let node_0 = &nodes[0];
    let sig_0 = node_0.sign(msg.to_vec());
    println!("Node 0 signature: {:?}", sig_0);
    signatures.insert(0, sig_0);

    let result = aggregator.aggregate_signatures(signatures.clone());
    assert!(result.is_err());

    // Step 3: Node 0 and Node 1 sign the message
    let node_1 = &nodes[1];
    let sig_1 = node_1.sign(msg.to_vec());
    println!("Node 1 signature: {:?}", sig_1);
    signatures.insert(1, sig_1);

    let sig = aggregator
        .aggregate_signatures(signatures.clone())
        .expect("not enough shares");
    println!("Combined signature: {:?}", sig);

    // Step 5: Verify the combined signature
    let result = sig.verify(&aggregator.public_key(), msg.to_vec());
    assert!(result);

    // Step 6: Node 2 signs the message, and the signature is verified
    let node_2 = &nodes[2];
    let sig_2 = node_2.sign(msg.to_vec());
    println!("Node 2 signature: {:?}", sig_2);
    signatures.insert(2, sig_2);

    let sig = aggregator
        .aggregate_signatures(signatures)
        .expect("not enough shares");
    let result = sig.verify(&aggregator.public_key(), msg.to_vec());
    assert!(result);
}
