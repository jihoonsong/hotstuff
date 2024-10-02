use hotstuff_mempool::{MempoolError, MempoolTransaction, TransactionPool};
use jsonrpsee::server::{RpcModule, Server, ServerHandle};
use std::{net::SocketAddr, sync::Arc};

use crate::{RpcApi, RpcApiServer, RpcConfig, RpcError};

pub struct RpcServer<P>
where
    P: TransactionPool<Transaction = MempoolTransaction, TransactionError = MempoolError>
        + Send
        + Sync
        + 'static,
{
    address: SocketAddr,
    transaction_pool: Arc<P>,
}

impl<P> RpcServer<P>
where
    P: TransactionPool<Transaction = MempoolTransaction, TransactionError = MempoolError>
        + Send
        + Sync
        + 'static,
{
    pub fn new(config: RpcConfig, transaction_pool: Arc<P>) -> Self {
        Self {
            address: config.address,
            transaction_pool,
        }
    }

    pub async fn build(&self) -> Result<ServerHandle, RpcError> {
        let rpc_api = RpcApi::new(self.transaction_pool.clone());

        let mut module = RpcModule::new(());
        module
            .merge(rpc_api.into_rpc())
            .map_err(|e| RpcError::Merge(String::from("rpc_api"), e))?;

        let server = Server::builder()
            .build(self.address)
            .await
            .map_err(|e| RpcError::Server(self.address, e))?;
        let server_handle = server.start(module);

        Ok(server_handle)
    }
}
