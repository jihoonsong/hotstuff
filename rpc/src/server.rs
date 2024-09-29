use jsonrpsee::server::{RpcModule, Server, ServerHandle};
use std::net::SocketAddr;

use crate::{HotStuffApi, HotStuffApiServer, RpcConfig, RpcError};

pub struct RpcServer {
    address: SocketAddr,
}

impl RpcServer {
    pub fn new(config: RpcConfig) -> Self {
        Self {
            address: config.address,
        }
    }

    pub async fn build(&self) -> Result<ServerHandle, RpcError> {
        let hotstuff_api = HotStuffApi::new();

        let mut module = RpcModule::new(());
        module
            .merge(hotstuff_api.into_rpc())
            .map_err(|e| RpcError::Merge(String::from("hotstuff_api"), e))?;

        let server = Server::builder()
            .build(self.address)
            .await
            .map_err(|e| RpcError::Server(self.address, e))?;
        let server_handle = server.start(module);

        Ok(server_handle)
    }
}
