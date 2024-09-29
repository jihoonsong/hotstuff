use std::net::SocketAddr;
use tokio::{net::TcpStream, sync::oneshot};
use tokio_util::bytes::Bytes;

pub(crate) enum CoordinatorMessage {
    DialablePeers {
        respond: oneshot::Sender<Vec<SocketAddr>>,
    },
    NewPeer {
        peer: SocketAddr,
        stream: TcpStream,
    },
}

pub enum HotStuffMessage {
    Dummy { data: Bytes },
}
