use hotstuff_p2p::{Decodable, Encodable, NetworkMessage, NetworkMessageHandler};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, error::SendError};
use tokio_util::bytes::Bytes;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub enum HotStuffMessage {
    Dummy {},
}

impl NetworkMessage for HotStuffMessage {}

impl Encodable for HotStuffMessage {
    fn encode(self) -> Bytes {
        Bytes::from(bincode::serialize(&self).unwrap())
    }
}
impl Decodable for HotStuffMessage {
    fn decode(data: Bytes) -> Self {
        bincode::deserialize(&data).unwrap()
    }
}

#[derive(Clone)]
pub struct HotStuffMessageHandler {
    pub to_hotstuff: mpsc::Sender<HotStuffMessage>,
}

impl NetworkMessageHandler<HotStuffMessage> for HotStuffMessageHandler {
    type NetworkMessageHandleError = SendError<HotStuffMessage>;

    async fn handle_message(
        &self,
        message: HotStuffMessage,
    ) -> Result<(), Self::NetworkMessageHandleError> {
        info!("Received a message: {:?}", message);
        self.to_hotstuff.send(message).await.unwrap();
        Ok(())
    }
}
