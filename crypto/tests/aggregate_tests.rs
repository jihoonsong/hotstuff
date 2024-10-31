use std::collections::HashMap;

mod utils;
use crate::utils::generate_random_keypairs;

#[test]
fn test_aggregate_signatures() {
    let (aggregator, keypairs) = generate_random_keypairs(1, 3);
    let message = b"test message".to_vec();

    let mut signatures = HashMap::new();
    for (i, keypair) in keypairs.iter().enumerate() {
        let signature = keypair.sign(message.clone());
        signatures.insert(i as u64, signature);
    }

    let result = aggregator.aggregate(signatures);
    assert!(result.is_ok());
    let aggregated_signature = result.unwrap();
    assert!(aggregated_signature.verify(&aggregator.public_key(), message));
}

#[test]
fn test_aggregate_signatures_insufficient() {
    let (aggregator, keypairs) = generate_random_keypairs(2, 3);
    let message = b"test message".to_vec();

    let mut signatures = HashMap::new();
    for (i, keypair) in keypairs.iter().take(1).enumerate() {
        let signature = keypair.sign(message.clone());
        signatures.insert(i as u64, signature);
    }

    let result = aggregator.aggregate(signatures);
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
        signatures.insert(((i + 1) % 3) as u64, signature);
    }

    let result = aggregator.aggregate(signatures);
    assert!(result.is_ok());
    let aggregated_signature = result.unwrap();
    assert!(!aggregated_signature.verify(&aggregator.public_key(), message));
}
