use blsttc::{Error, PublicKeySet, PublicKeyShare, Signature, SignatureShare, PK_SIZE, SIG_SIZE};
use hex;
use std::collections::HashMap;

pub fn verify(pk_share: String, msg: &[u8], signature_share: String) -> bool {
    let pk_share: [u8; PK_SIZE] = hex::decode(pk_share).unwrap().try_into().unwrap();
    let pk_share = PublicKeyShare::from_bytes(pk_share).unwrap();

    let signature_share: [u8; SIG_SIZE] = hex::decode(signature_share).unwrap().try_into().unwrap();
    let signature_share = SignatureShare::from_bytes(signature_share).unwrap();
    pk_share.verify(&signature_share, msg)
}

pub fn combine_signatures(
    pk_set: String,
    signature_shares: HashMap<usize, String>,
) -> Result<Signature, Error> {
    let pk_set = PublicKeySet::from_bytes(hex::decode(pk_set).unwrap()).unwrap();

    let signature_shares: HashMap<usize, SignatureShare> =
        signature_shares
            .iter()
            .fold(HashMap::new(), |mut acc, (id, signature)| {
                let signature_share: [u8; SIG_SIZE] =
                    hex::decode(signature).unwrap().try_into().unwrap();
                acc.insert(*id, SignatureShare::from_bytes(signature_share).unwrap());
                acc
            });
    pk_set.combine_signatures(signature_shares)
}
