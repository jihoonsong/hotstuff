use futures::{SinkExt, StreamExt};
use hotstuff_crypto::PublicKey;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio_util::bytes::Bytes;

use crate::{Decodable, Encodable, NetworkError, Reader, Writer};

#[derive(Clone, Serialize, Deserialize)]
pub struct Handshake {
    pub message: HandshakeMessage,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct HandshakeMessage {
    pub identity: PublicKey,
}

impl Encodable for Handshake {
    fn encode(self) -> Bytes {
        Bytes::from(bincode::serialize(&self).unwrap())
    }
}

impl Decodable for Handshake {
    fn decode(data: Bytes) -> Self {
        bincode::deserialize(&data).unwrap()
    }
}

impl Handshake {
    pub fn new(identity: PublicKey) -> Self {
        Self {
            message: HandshakeMessage { identity },
        }
    }

    pub async fn exchange(
        &self,
        writer: &mut Writer,
        reader: &mut Reader,
        handshake_timeout: Duration,
    ) -> Result<HandshakeMessage, NetworkError> {
        let sender_task = async {
            writer
                .send(self.clone().encode())
                .await
                .map_err(NetworkError::Handshake)
        };

        let receiver_task = async {
            let frame = reader
                .next()
                .await
                .ok_or_else(|| NetworkError::HandshakeClosed)?;
            let data = frame.map_err(NetworkError::Handshake)?;
            Ok(Handshake::decode(data.freeze()).message)
        };

        let exchange_handshake = async { tokio::try_join!(sender_task, receiver_task) };

        match tokio::time::timeout(handshake_timeout, exchange_handshake).await {
            Ok(Ok((_, message))) => Ok(message),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(NetworkError::HandshakeTimeout),
        }
    }
}
