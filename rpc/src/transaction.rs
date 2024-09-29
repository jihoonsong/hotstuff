use std::future::Future;

use crate::{HotStuffError, TransactionRequest};

pub trait HotStuffTransaction {
    fn send_transaction(
        &self,
        request: TransactionRequest,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send
    where
        Self: HotStuffError,
    {
        async move { Ok(format!("send_transaction received {:?}", request)) }
    }
}
