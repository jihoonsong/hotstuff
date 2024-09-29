use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use jsonrpsee_types::ErrorObject;
use tracing::info;

use crate::{
    HotStuffApiError, HotStuffApis, HotStuffError, HotStuffTransaction, TransactionRequest,
};

pub struct HotStuffApi {}

impl HotStuffApi {
    pub fn new() -> Self {
        Self {}
    }
}
#[rpc(server, namespace = "hotstuff")]
pub trait HotStuffApi {
    #[method(name = "sendTransaction")]
    async fn send_transaction(&self, request: TransactionRequest) -> RpcResult<String>;
}

#[async_trait::async_trait]
impl<T> HotStuffApiServer for T
where
    T: HotStuffApis,
    ErrorObject<'static>: From<T::Error>,
{
    async fn send_transaction(&self, request: TransactionRequest) -> RpcResult<String> {
        info!("send_transaction: {:?}", request);
        Ok(HotStuffTransaction::send_transaction(self, request).await?)
    }
}

impl HotStuffError for HotStuffApi {
    type Error = HotStuffApiError;
}

impl HotStuffTransaction for HotStuffApi {}
