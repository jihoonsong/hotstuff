use futures::{stream::SplitStream, StreamExt};
use std::{marker::PhantomData, net::SocketAddr};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::debug;

use crate::{NetworkError, NetworkMessage, NetworkMessageHandler};

pub struct Peer<M, H>
where
    M: NetworkMessage,
    H: NetworkMessageHandler<M>,
{
    identity: SocketAddr, // TODO: Use cryptographic public key.
    reader: Reader,
    peer_message_handler: H,
    _marker: PhantomData<M>,
}

type Reader = SplitStream<Framed<TcpStream, LengthDelimitedCodec>>;

impl<M, H> Peer<M, H>
where
    M: NetworkMessage,
    H: NetworkMessageHandler<M>,
{
    pub fn new(identity: SocketAddr, reader: Reader, peer_message_handler: H) -> Self {
        Self {
            identity,
            reader,
            peer_message_handler,
            _marker: PhantomData,
        }
    }

    pub async fn run(mut self) {
        while let Some(frame) = self.reader.next().await {
            match frame.map_err(|e| NetworkError::ReceiveMessage(self.identity, e)) {
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
