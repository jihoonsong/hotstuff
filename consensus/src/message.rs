use hotstuff_mempool::Transaction;
use hotstuff_p2p::{Decodable, Encodable, NetworkMessage, NetworkMessageHandler};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, error::SendError};
use tokio_util::bytes::Bytes;
use tracing::info;

use crate::Block;

#[derive(Debug, Serialize, Deserialize)]
pub enum HotStuffMessage<T> {
    Proposal(Block<T>),
}

impl<T> NetworkMessage for HotStuffMessage<T> where T: Transaction {}

impl<T> Encodable for HotStuffMessage<T>
where
    T: Transaction,
{
    fn encode(self) -> Bytes {
        Bytes::from(bincode::serialize(&self).unwrap())
    }
}
impl<T> Decodable for HotStuffMessage<T>
where
    T: Transaction,
{
    fn decode(data: Bytes) -> Self {
        bincode::deserialize(&data).unwrap()
    }
}

#[derive(Clone)]
pub struct HotStuffMessageHandler<T>
where
    T: Transaction,
{
    pub to_hotstuff: mpsc::Sender<HotStuffMessage<T>>,
}

impl<T> NetworkMessageHandler<HotStuffMessage<T>> for HotStuffMessageHandler<T>
where
    T: Transaction,
{
    type NetworkMessageHandleError = SendError<HotStuffMessage<T>>;

    async fn handle_message(
        &self,
        message: HotStuffMessage<T>,
    ) -> Result<(), Self::NetworkMessageHandleError> {
        info!("Received a message: {:?}", message);
        self.to_hotstuff.send(message).await.unwrap();
        Ok(())
    }
}
