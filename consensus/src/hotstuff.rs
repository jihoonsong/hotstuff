use hotstuff_mempool::{Transaction, TransactionPool};
use hotstuff_p2p::NetworkAction;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;

use crate::{HotStuffConfig, HotStuffMessage, HotStuffMessageHandler};

pub struct HotStuff<T, P>
where
    T: Transaction,
    P: TransactionPool<Transaction = T>,
{
    from_hotstuff: mpsc::Receiver<HotStuffMessage>,
    handler: HotStuffMessageHandler,
    mempool: Arc<P>,
    to_network: Option<mpsc::Sender<NetworkAction>>,
}

impl<T, P> HotStuff<T, P>
where
    T: Transaction,
    P: TransactionPool<Transaction = T>,
{
    pub fn new(config: HotStuffConfig, mempool: P) -> Self {
        let (to_hotstuff, from_hotstuff) = mpsc::channel(config.mailbox_size);
        let handler = HotStuffMessageHandler { to_hotstuff };

        Self {
            from_hotstuff,
            handler,
            mempool: Arc::new(mempool),
            to_network: None,
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
                // TODO: Timer comes here.
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
}
