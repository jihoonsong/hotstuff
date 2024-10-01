use hotstuff_mempool::MempoolTransaction;

use crate::TransactionRequest;

pub(crate) fn to_transaction(request: TransactionRequest) -> Option<MempoolTransaction> {
    Some(match request {
        TransactionRequest::HotStuff(request) => MempoolTransaction {
            nonce: request.nonce,
            data: request.data,
        },
    })
}
