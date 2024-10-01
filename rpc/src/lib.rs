mod api;
mod config;
mod convert;
mod error;
mod request;
mod server;
mod transaction;

pub use config::RpcConfig;
pub use error::RpcError;
pub use request::{MempoolTransactionRequest, TransactionRequest};
pub use server::RpcServer;

pub(crate) use api::{RpcApi, RpcApiServer};
pub(crate) use convert::to_transaction;
pub(crate) use error::{FromRpcError, RpcApiError};
pub(crate) use transaction::RpcApiTransaction;

pub(crate) trait RpcApis: RpcApiError + RpcApiTransaction + 'static {}

impl<T> RpcApis for T where T: RpcApiError + RpcApiTransaction + 'static {}
