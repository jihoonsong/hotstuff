use futures::StreamExt;
use hotstuff_crypto::PublicKey;
use std::marker::PhantomData;
use tokio::sync::mpsc;
use tracing::debug;

use crate::{NetworkError, NetworkMessage, NetworkMessageHandler, PeerManagerMessage, Reader};

pub struct Peer<M, H>
where
    M: NetworkMessage,
    H: NetworkMessageHandler<M>,
{
    identity: PublicKey,
    reader: Reader,
    peer_message_handler: H,
    to_peer_manager: mpsc::Sender<PeerManagerMessage>,
    _marker: PhantomData<M>,
}

impl<M, H> Peer<M, H>
where
    M: NetworkMessage,
    H: NetworkMessageHandler<M>,
{
    pub fn new(
        identity: PublicKey,
        reader: Reader,
        peer_message_handler: H,
        to_peer_manager: mpsc::Sender<PeerManagerMessage>,
    ) -> Self {
        Self {
            identity,
            reader,
            peer_message_handler,
            to_peer_manager,
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
                    debug!("Disconnected from peer {}: {}", self.identity, e);
                    self.to_peer_manager
                        .send(PeerManagerMessage::DisconnectedPeer {
                            identity: self.identity,
                        })
                        .await
                        .unwrap();
                    break;
                }
            }
        }
    }
}
