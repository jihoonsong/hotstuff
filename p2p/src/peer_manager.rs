use futures::{future::join_all, stream::SplitSink, SinkExt, StreamExt};
use std::{collections::HashMap, marker::PhantomData, net::SocketAddr};
use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
    sync::{mpsc, oneshot},
};
use tokio_util::{
    bytes::Bytes,
    codec::{Framed, LengthDelimitedCodec},
};
use tracing::info;

use crate::{
    NetworkAction, NetworkMessage, NetworkMessageHandler, Peer, PeerManagerConfig,
    PeerManagerMessage,
};

type Writer = SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>;

pub struct PeerManager<M, H>
where
    M: NetworkMessage,
    H: NetworkMessageHandler<M>,
{
    min_peers: u16,
    max_peers: u16,
    candidate_peers: Vec<SocketAddr>,
    connected_peers: HashMap<SocketAddr, Writer>,
    to_peer_manager: mpsc::Sender<PeerManagerMessage>,
    from_peer_manager: mpsc::Receiver<PeerManagerMessage>,
    peer_message_handler: H,
    _marker: PhantomData<M>,
}

impl<M, H> PeerManager<M, H>
where
    M: NetworkMessage,
    H: NetworkMessageHandler<M>,
{
    pub fn new(config: PeerManagerConfig, peer_message_handler: H) -> Self {
        let (to_peer_manager, from_peer_manager) = mpsc::channel(config.mailbox_size);

        Self {
            min_peers: config.min_peers,
            max_peers: config.max_peers,
            candidate_peers: config.peers.unwrap_or_default(),
            connected_peers: HashMap::new(),
            to_peer_manager,
            from_peer_manager,
            peer_message_handler,
            _marker: PhantomData,
        }
    }

    pub async fn run(mut self) {
        while let Some(message) = self.from_peer_manager.recv().await {
            match message {
                PeerManagerMessage::DialablePeers { respond } => {
                    respond.send(self.dialable_peers()).unwrap();
                }
                PeerManagerMessage::NewPeer { peer, stream } => {
                    self.new_peer(peer, stream).await;
                }
                PeerManagerMessage::NetworkAction(NetworkAction::IsReady { respond }) => {
                    self.is_ready(respond).await;
                }
                PeerManagerMessage::NetworkAction(NetworkAction::Send { recipient, message }) => {
                    self.send(recipient, message).await;
                }
                PeerManagerMessage::NetworkAction(NetworkAction::Broadcast { message }) => {
                    self.broadcast(message).await;
                }
            }
        }
    }

    pub fn mailbox(&self) -> mpsc::Sender<PeerManagerMessage> {
        self.to_peer_manager.clone()
    }

    fn dialable_peers(&self) -> Vec<SocketAddr> {
        self.candidate_peers
            .iter()
            .filter(|peer| !self.connected_peers.contains_key(peer))
            .take(self.max_peers as usize - self.connected_peers.len())
            .cloned()
            .collect()
    }

    async fn new_peer(&mut self, peer: SocketAddr, mut stream: TcpStream) {
        // If we have reached the maximum number of peers, just close the new connection.
        if self.connected_peers.len() >= self.max_peers as usize {
            let _ = stream.shutdown().await;
            info!("Shutdown new connection with {peer}: max peers reached");
            return;
        }

        // Split the communication channel into writer and reader.
        let framed = Framed::new(stream, LengthDelimitedCodec::new());
        let (writer, reader) = framed.split();

        // TODO: After we have cryptography crate, we should use node's PublicKey as its identity.
        // We can use PublicKey to determine if the node is already connected. A handshake should
        // happen in advance to exchange identities.
        self.connected_peers.insert(peer, writer);

        // Spawn peer listening to its messages. Received messages will be redirected to Consensus.
        let peer_message_handler = self.peer_message_handler.clone();
        tokio::spawn(async move {
            Peer::new(peer, reader, peer_message_handler).run().await;
        });

        info!("New connected peer: {peer}");
    }

    async fn is_ready(&mut self, respond: oneshot::Sender<bool>) {
        respond
            .send(self.connected_peers.len() >= self.min_peers as usize)
            .unwrap();
    }

    async fn send(&mut self, recipient: SocketAddr, message: Bytes) {
        let send_futures = self
            .connected_peers
            .iter_mut()
            .filter(|(peer, _)| **peer == recipient)
            .map(|(_, writer)| writer.send(message.clone()));
        join_all(send_futures).await;
    }

    async fn broadcast(&mut self, message: Bytes) {
        let broadcast_futures = self
            .connected_peers
            .iter_mut()
            .map(|(_, writer)| writer.send(message.clone()));
        join_all(broadcast_futures).await;
    }
}
