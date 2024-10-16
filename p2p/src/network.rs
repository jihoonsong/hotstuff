use std::marker::PhantomData;
use tokio::sync::mpsc;
use tracing::info;

use crate::{
    message::{NetworkAction, NetworkMessage, NetworkMessageHandler},
    Dialer, Listener, NetworkConfig, PeerManager, PeerManagerMessage,
};

pub struct P2PNetwork<M, H>
where
    M: NetworkMessage,
    H: NetworkMessageHandler<M>,
{
    configs: NetworkConfig,
    hotstuff: mpsc::Sender<HotStuffMessage>,
    peer_manager: Option<PeerManager<M, H>>,
    peer_message_handler: H,
    _marker: PhantomData<M>,
}

impl<M, H> P2PNetwork<M, H>
where
    M: NetworkMessage,
    H: NetworkMessageHandler<M>,
{
    pub fn new(config: NetworkConfig, peer_message_handler: H) -> Self {
        let (to_p2p_network, from_p2p_network) = mpsc::channel(config.mailbox_size);

        Self {
            configs: config,
            hotstuff,
            to_p2p_network,
            from_p2p_network,
            peer_manager: None,
            peer_message_handler,
            _marker: PhantomData,
        }
    }

    pub async fn run(mut self) {
        // Run a `PeerManager` to manage connected peers.
        if self.peer_manager.is_none() {
            self.peer_manager = Some(PeerManager::new(
                self.configs.peer_manager,
                self.peer_message_handler,
            ));
        }
        let peer_manager = self.peer_manager.unwrap();
        let peer_manager_mailbox = peer_manager.mailbox();
        let mut peer_manager_task = tokio::spawn(peer_manager.run());

        // Run a dialer to dial peers periodically.
        let dialer = Dialer::new(self.configs.dialer, peer_manager_mailbox.clone());
        let mut dialer_task = tokio::spawn(dialer.run());

        // Run a listener to accept incoming connections.
        let listener = Listener::new(self.configs.listener, peer_manager_mailbox.clone());
        let mut listener_task = tokio::spawn(listener.run());

        match tokio::try_join!(&mut peer_manager_task, &mut dialer_task, &mut listener_task) {
            Ok(_) => {
                info!("P2P network tasks completed");
            }
            Err(e) => {
                info!(error=?e, "An error occured while running P2P network");
                peer_manager_task.abort();
                dialer_task.abort();
                listener_task.abort();
            }
        };
    }
}

