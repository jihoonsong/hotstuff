use futures::StreamExt;
use hotstuff_crypto::PublicKey;
use std::marker::PhantomData;
use tracing::debug;

use crate::{NetworkError, NetworkMessage, NetworkMessageHandler, Reader};

pub struct Peer<M, H>
where
    M: NetworkMessage,
    H: NetworkMessageHandler<M>,
{
    identity: PublicKey,
    reader: Reader,
    peer_message_handler: H,
    _marker: PhantomData<M>,
}

impl<M, H> Peer<M, H>
where
    M: NetworkMessage,
    H: NetworkMessageHandler<M>,
{
    pub fn new(identity: PublicKey, reader: Reader, peer_message_handler: H) -> Self {
        Self {
            identity,
            reader,
            peer_message_handler,
            _marker: PhantomData,
        }
    }

    pub async fn run(mut self) {
        while let Some(frame) = self.reader.next().await {
            match frame.map_err(|e| NetworkError::ReceiveMessage(self.identity.clone(), e)) {
                Ok(data) => {
                    self.peer_message_handler
                        .handle_message(M::decode(data.freeze()))
                        .await
                        .unwrap();
                }
                Err(e) => {
                    debug!(error=?e);
                    break;
                }
            }
        }
    }
}
