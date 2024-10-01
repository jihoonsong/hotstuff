use hotstuff_mempool::{MempoolTransaction, TransactionKind};

use crate::TransactionRequest;

pub(crate) fn to_transaction(request: TransactionRequest) -> Option<MempoolTransaction> {
    Some(match request {
        TransactionRequest::Mempool(request) => MempoolTransaction {
            nonce: request.nonce,
            data: request.data,
            kind: TransactionKind::Mempool,
        },
    })
}
