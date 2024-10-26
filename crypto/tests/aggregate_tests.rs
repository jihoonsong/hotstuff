use std::collections::HashMap;

use hotstuff_crypto::{generate_random_keypairs, Aggregator};

#[test]
fn test_aggregate_signatures() {
    let (aggregator, keypairs) = generate_random_keypairs(1, 3);
    let message = b"test message".to_vec();

    let mut signatures = HashMap::new();
    for (i, keypair) in keypairs.iter().enumerate() {
        let signature = keypair.sign(message.clone());
        signatures.insert(i, signature);
    }

    let result = aggregator.aggregate_signatures(signatures);
    assert!(result.is_ok());
    let aggregated_signature = result.unwrap();
    assert!(aggregated_signature.verify(message, aggregator.pubkey()));
}

#[test]
fn test_aggregate_signatures_insufficient() {
    let (public_key_set, keypairs) = generate_random_keypairs(2, 3);
    let aggregator = Aggregator::new(public_key_set.to_bytes());
    let message = b"test message".to_vec();

    let mut signatures = HashMap::new();
    for (i, keypair) in keypairs.iter().take(1).enumerate() {
        let signature = keypair.sign(message.clone());
        signatures.insert(i, signature);
    }

    let result = aggregator.aggregate_signatures(signatures);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Not enough shares"));
}

#[test]
fn test_aggregate_signatures_wrong_id() {
    let (aggregator, keypairs) = generate_random_keypairs(1, 3);
    let message = b"test message".to_vec();

    let mut signatures = HashMap::new();
    for (i, keypair) in keypairs.iter().enumerate() {
        let signature = keypair.sign(message.clone());
        // Insert the signature with the wrong ID
        signatures.insert((i + 1) % 3, signature);
    }

    let result = aggregator.aggregate_signatures(signatures);
    assert!(result.is_ok());
    let aggregated_signature = result.unwrap();
    assert!(aggregated_signature.verify(message, aggregator.pubkey()) == false);
}
