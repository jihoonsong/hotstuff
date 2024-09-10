use crate::{
    api::{TransactionApi, TransactionServer},
    error::RPCError,
};

use jsonrpsee::server::{RpcModule, Server, ServerHandle};
use std::net::SocketAddr;

pub struct RpcServer {
    address: SocketAddr,
}

impl RpcServer {
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
    }

    pub async fn build(&self) -> Result<ServerHandle, RPCError> {
        let transaction_api = TransactionApi::new();

        let mut module = RpcModule::new(());
        module
            .merge(transaction_api.into_rpc())
            .map_err(|e| RPCError::Merge(String::from("transaction_api"), e))?;

        let server = Server::builder()
            .build(self.address)
            .await
            .map_err(|e| RPCError::Server(self.address, e))?;
        let server_handle = server.start(module);

        Ok(server_handle)
    }
}
