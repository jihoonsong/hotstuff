use tokio::sync::mpsc;
use tracing::info;

use crate::{
    Coordinator, CoordinatorConfig, Dialer, DialerConfig, HotStuffMessage, Listener,
    ListenerConfig, NetworkConfig,
};

pub struct Network {
    coordinator: CoordinatorConfig,
    dialer: DialerConfig,
    listener: ListenerConfig,
    hotstuff: mpsc::Sender<HotStuffMessage>,
}

impl Network {
    pub fn new(config: NetworkConfig, hotstuff: mpsc::Sender<HotStuffMessage>) -> Self {
        Self {
            coordinator: config.coordinator,
            dialer: config.dialer,
            listener: config.listener,
            hotstuff,
        }
    }

    pub async fn run(self) {
        // Run peer manager to manage connected peers.
        let coordinator = Coordinator::new(self.coordinator, self.hotstuff);
        let coordinator_mailbox = coordinator.mailbox();
        let mut coordinator_task = tokio::spawn(coordinator.run());

        // Run dialer to dial peers periodically.
        let dialer = Dialer::new(self.dialer, coordinator_mailbox.clone());
        let mut dialer_task = tokio::spawn(dialer.run());

        // Run listener to accept incoming connections.
        let listener = Listener::new(self.listener, coordinator_mailbox.clone());
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
