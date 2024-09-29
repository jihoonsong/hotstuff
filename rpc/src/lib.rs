mod api;
mod config;
mod error;
mod request;
mod server;
mod transaction;

pub use config::RpcConfig;
pub use request::TransactionRequest;
pub use server::RpcServer;

pub(crate) use api::{HotStuffApi, HotStuffApiServer};
pub(crate) use error::{HotStuffApiError, HotStuffError, RpcError};
pub(crate) use transaction::HotStuffTransaction;

pub(crate) trait HotStuffApis: HotStuffError + HotStuffTransaction + 'static {}

impl<T> HotStuffApis for T where T: HotStuffError + HotStuffTransaction + 'static {}
