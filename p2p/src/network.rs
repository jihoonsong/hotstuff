use hotstuff_consensus::HotStuffMessage;
use tokio::sync::mpsc;
use tracing::info;

use crate::{Coordinator, Dialer, Listener, NetworkConfig};

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
        let coordinator = Coordinator::new(self.configs.coordinator, self.hotstuff);
        let coordinator_mailbox = coordinator.mailbox();
        let mut coordinator_task = tokio::spawn(coordinator.run());

        // Run dialer to dial peers periodically.
        let dialer = Dialer::new(self.configs.dialer, coordinator_mailbox.clone());
        let mut dialer_task = tokio::spawn(dialer.run());

        // Run listener to accept incoming connections.
        let listener = Listener::new(self.configs.listener, coordinator_mailbox.clone());
        let mut listener_task = tokio::spawn(listener.run());

        match tokio::try_join!(&mut coordinator_task, &mut dialer_task, &mut listener_task) {
            Ok(_) => {
                info!("P2P network tasks completed");
            }
            Err(e) => {
                info!(error=?e, "An error occured while running P2P network");
                coordinator_task.abort();
                dialer_task.abort();
                listener_task.abort();
            }
        };
    }
}
