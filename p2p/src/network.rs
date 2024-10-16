use hotstuff_consensus::HotStuffMessage;
use tokio::sync::mpsc;
use tracing::info;

use crate::{PeerManager, Dialer, Listener, NetworkConfig};

pub struct Network {
    configs: NetworkConfig,
    hotstuff: mpsc::Sender<HotStuffMessage>,
}

impl Network {
    pub fn new(config: NetworkConfig, hotstuff: mpsc::Sender<HotStuffMessage>) -> Self {
        Self {
            configs: config,
            hotstuff,
        }
    }

    pub async fn run(self) {
        // Run peer manager to manage connected peers.
        let peer_manager = PeerManager::new(self.configs.peer_manager, self.hotstuff);
        let peer_manager_mailbox = peer_manager.mailbox();
        let mut peer_manager_task = tokio::spawn(peer_manager.run());

        // Run dialer to dial peers periodically.
        let dialer = Dialer::new(self.configs.dialer, peer_manager_mailbox.clone());
        let mut dialer_task = tokio::spawn(dialer.run());

        // Run listener to accept incoming connections.
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

