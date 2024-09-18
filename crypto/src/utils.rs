use blsttc::{Error, PublicKeySet, PublicKeyShare, Signature, SignatureShare};
use std::collections::HashMap;

pub fn verify(pk_share: PublicKeyShare, msg: &[u8], signature: &SignatureShare) -> bool {
    pk_share.verify(signature, msg)
}

pub fn combine_signatures(
    pk_set: PublicKeySet,
    signature_shares: HashMap<usize, SignatureShare>,
) -> Result<Signature, Error> {
    pk_set.combine_signatures(signature_shares)
}
