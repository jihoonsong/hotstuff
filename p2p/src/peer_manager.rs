use futures::{stream::SplitSink, StreamExt};
use hotstuff_consensus::HotStuffMessage;
use std::{collections::HashMap, net::SocketAddr};
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::mpsc};
use tokio_util::{
    bytes::Bytes,
    codec::{Framed, LengthDelimitedCodec},
};
use tracing::info;

use crate::{PeerManagerConfig, PeerManagerMessage, Peer};

type Writer = SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>;

pub struct PeerManager {
    max_peers: u16,
    candidate_peers: Vec<SocketAddr>,
    connected_peers: HashMap<SocketAddr, Writer>,
    dispatcher: mpsc::Sender<PeerManagerMessage>,
    mailbox: mpsc::Receiver<PeerManagerMessage>,
    hotstuff: mpsc::Sender<HotStuffMessage>,
}

impl PeerManager {
    pub fn new(config: PeerManagerConfig, hotstuff: mpsc::Sender<HotStuffMessage>) -> Self {
        let (dispatcher, mailbox) = mpsc::channel(config.mailbox_size);

        Self {
            max_peers: config.max_peers,
            candidate_peers: config.peers.unwrap_or_default(),
            connected_peers: HashMap::new(),
            dispatcher,
            mailbox,
            hotstuff,
        }
    }

    pub async fn run(mut self) {
        while let Some(message) = self.mailbox.recv().await {
            match message {
                PeerManagerMessage::DialablePeers { respond } => {
                    respond.send(self.dialable_peers()).unwrap();
                }
                PeerManagerMessage::NewPeer { peer, stream } => {
                    self.new_peer(peer, stream).await;
                }
            }
        }
    }

    pub fn mailbox(&self) -> mpsc::Sender<PeerManagerMessage> {
        self.dispatcher.clone()
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
        let hotstuff = self.hotstuff.clone();
        tokio::spawn(async move {
            Peer::new(peer, reader, hotstuff).run().await;
        });

        info!("New connected peer: {peer}");
    }
}

