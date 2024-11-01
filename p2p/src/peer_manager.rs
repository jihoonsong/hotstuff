use futures::{future::join_all, SinkExt, StreamExt};
use hotstuff_crypto::PublicKey;
use std::{collections::HashMap, marker::PhantomData, net::SocketAddr, time::Duration};
use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
    sync::{mpsc, oneshot},
};
use tokio_util::{
    bytes::Bytes,
    codec::{Framed, LengthDelimitedCodec},
};
use tracing::{debug, info};

use crate::{
    Handshake, HandshakeMessage, NetworkAction, NetworkMessage, NetworkMessageHandler, Peer,
    PeerManagerConfig, PeerManagerMessage, Writer,
};

pub struct PeerManager<M, H>
where
    M: NetworkMessage,
    H: NetworkMessageHandler<M>,
{
    min_peers: u16,
    max_peers: u16,
    candidate_peers: HashMap<PublicKey, SocketAddr>,
    connected_peers: HashMap<PublicKey, Writer>,
    to_peer_manager: mpsc::Sender<PeerManagerMessage>,
    from_peer_manager: mpsc::Receiver<PeerManagerMessage>,
    peer_message_handler: H,
    identity: PublicKey,
    handshake_timeout: Duration,
    _marker: PhantomData<M>,
}

impl<M, H> PeerManager<M, H>
where
    M: NetworkMessage,
    H: NetworkMessageHandler<M>,
{
    pub fn new(config: PeerManagerConfig, peer_message_handler: H, identity: PublicKey) -> Self {
        let (to_peer_manager, from_peer_manager) = mpsc::channel(config.mailbox_size);
        let handshake_timeout = Duration::from_millis(config.handshake_timeout);

        Self {
            min_peers: config.min_peers,
            max_peers: config.max_peers,
            candidate_peers: config.peers(),
            connected_peers: HashMap::new(),
            to_peer_manager,
            from_peer_manager,
            peer_message_handler,
            identity,
            handshake_timeout,
            _marker: PhantomData,
        }
    }

    pub async fn run(mut self) {
        while let Some(message) = self.from_peer_manager.recv().await {
            match message {
                PeerManagerMessage::DialablePeers { reply } => {
                    reply.send(self.dialable_peers()).unwrap();
                }
                PeerManagerMessage::NewPeer { address, stream } => {
                    self.new_peer(address, stream).await;
                }
                PeerManagerMessage::DisconnectedPeer { identity } => {
                    self.disconnected_peer(identity).await;
                }
                PeerManagerMessage::NetworkAction(NetworkAction::IsReady { reply }) => {
                    self.is_ready(reply).await;
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
            .filter(|&(peer, _)| (!self.connected_peers.contains_key(peer)))
            .map(|(_, &address)| address)
            .take(self.max_peers as usize - self.connected_peers.len())
            .collect()
    }

    async fn new_peer(&mut self, address: SocketAddr, mut stream: TcpStream) {
        // If we have reached the maximum number of peers, just close the new connection.
        if self.connected_peers.len() >= self.max_peers as usize {
            let _ = stream.shutdown().await;
            info!("Shutdown new connection with {address}: max peers reached");
            return;
        }

        // Split the communication channel into writer and reader.
        let framed = Framed::new(stream, LengthDelimitedCodec::new());
        let (mut writer, mut reader) = framed.split();

        // Exchange handshake with the new peer.
        let handshake = Handshake::new(self.identity.clone());
        let HandshakeMessage {
            identity: peer_identity,
        } = match handshake
            .exchange(&mut writer, &mut reader, self.handshake_timeout)
            .await
        {
            Ok(message) => message,
            Err(e) => {
                debug!("Failed to exchange handshake: {e}");
                return;
            }
        };

        // If the peer is already connected, just close the connection.
        if self.connected_peers.contains_key(&peer_identity) {
            let _ = writer.close().await;
            info!("Shutdown new connection with {address}: peer already connected");
            return;
        }
        info!("{}: New connected peer: {}", self.identity, peer_identity);

        // Add the peer to the connected peers.
        self.connected_peers.insert(peer_identity.clone(), writer);

        // Spawn peer listening to its messages. Received messages will be redirected to Consensus.
        let peer_message_handler = self.peer_message_handler.clone();
        let to_peer_manager = self.to_peer_manager.clone();
        tokio::spawn(async move {
            Peer::new(peer_identity, reader, peer_message_handler, to_peer_manager)
                .run()
                .await;
        });
    }

    async fn disconnected_peer(&mut self, identity: PublicKey) {
        self.connected_peers.remove(&identity);
    }

    async fn is_ready(&mut self, reply: oneshot::Sender<bool>) {
        reply
            .send(self.connected_peers.len() >= self.min_peers as usize)
            .unwrap();
    }

    async fn send(&mut self, recipient: PublicKey, message: Bytes) {
        // Loopback the message if the recipient is myself.
        if recipient == self.identity {
            self.peer_message_handler
                .handle_message(M::decode(message))
                .await
                .unwrap();
            return;
        }

        // Send the message to the recipient.
        let send_futures = self
            .connected_peers
            .iter_mut()
            .filter(|&(peer, _)| *peer == recipient)
            .map(|(_, writer)| writer.send(message.clone()));
        join_all(send_futures).await;
    }

    async fn broadcast(&mut self, message: Bytes) {
        // Send the message to myself also.
        self.peer_message_handler
            .handle_message(M::decode(message.clone()))
            .await
            .unwrap();

        // Broadcast the message to all connected peers.
        let broadcast_futures = self
            .connected_peers
            .iter_mut()
            .map(|(_, writer)| writer.send(message.clone()));
        join_all(broadcast_futures).await;
    }
}
