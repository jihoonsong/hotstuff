use hotstuff_mempool::{Transaction, TransactionPool};
use hotstuff_p2p::NetworkAction;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;

use crate::{HotStuffConfig, HotStuffMessage, HotStuffMessageHandler, LeaderElector, Timeout};

pub struct HotStuff<T, P, L>
where
    T: Transaction,
    P: TransactionPool<Transaction = T>,
    L: LeaderElector,
{
    from_hotstuff: mpsc::Receiver<HotStuffMessage>,
    handler: HotStuffMessageHandler,
    mempool: Arc<P>,
    to_network: Option<mpsc::Sender<NetworkAction>>,
    timeout: Timeout,
    leader_elector: L,
}

impl<T, P, L> HotStuff<T, P, L>
where
    T: Transaction,
    P: TransactionPool<Transaction = T>,
    L: LeaderElector,
{
    pub fn new(config: HotStuffConfig, mempool: P, leader_elector: L) -> Self {
        let (to_hotstuff, from_hotstuff) = mpsc::channel(config.mailbox_size);
        let handler = HotStuffMessageHandler { to_hotstuff };
        let timeout = Timeout::new(config.timeout);

        Self {
            from_hotstuff,
            handler,
            mempool: Arc::new(mempool),
            to_network: None,
            timeout,
            leader_elector,
        }
    }

    pub async fn run(mut self) {
        loop {
            tokio::select! {
                Some(message) = self.from_hotstuff.recv() => match message {
                    HotStuffMessage::Dummy {} => {
                        info!("Received dummy message");
                    }
                },
                () = &mut self.timeout => self.timeout().await,
            }
        }
    }

    pub fn handler(&self) -> HotStuffMessageHandler {
        self.handler.clone()
    }

    pub fn mempool(&self) -> Arc<P> {
        self.mempool.clone()
    }

    pub fn set_network(&mut self, network: mpsc::Sender<NetworkAction>) {
        self.to_network = Some(network);
    }

    async fn timeout(&self) {
        info!("timeout!");
    }
}
