use hotstuff_crypto::PublicKey;
use std::{fmt::Debug, future::Future, net::SocketAddr};
use tokio::{net::TcpStream, sync::oneshot};
use tokio_util::bytes::Bytes;

pub trait Encodable {
    fn encode(self) -> Bytes;
}

pub trait Decodable {
    fn decode(data: Bytes) -> Self;
}

pub trait NetworkMessage: Encodable + Decodable + Send + Sync + 'static {}

pub trait NetworkMessageHandler<M>: Clone + Send + Sync + 'static
where
    M: NetworkMessage,
{
    type NetworkMessageHandleError: Debug;

    fn handle_message(
        &self,
        message: M,
    ) -> impl Future<Output = Result<(), Self::NetworkMessageHandleError>> + Send;
}

pub enum NetworkAction {
    IsReady {
        reply: oneshot::Sender<bool>,
    },
    Send {
        recipient: PublicKey,
        message: Bytes,
    },
    Broadcast {
        message: Bytes,
    },
}

pub(crate) enum PeerManagerMessage {
    DialablePeers {
        reply: oneshot::Sender<Vec<SocketAddr>>,
    },
    NewPeer {
        address: SocketAddr,
        stream: TcpStream,
    },
    DisconnectedPeer {
        identity: PublicKey,
    },
    NetworkAction(NetworkAction),
}
