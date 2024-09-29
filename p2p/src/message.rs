use std::net::SocketAddr;
use tokio::{net::TcpStream, sync::oneshot};

pub(crate) enum CoordinatorMessage {
    DialablePeers {
        respond: oneshot::Sender<Vec<SocketAddr>>,
    },
    NewPeer {
        peer: SocketAddr,
        stream: TcpStream,
    },
}
