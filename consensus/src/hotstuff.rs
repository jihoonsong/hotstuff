use hotstuff_mempool::{Transaction, TransactionPool};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;

use crate::{message::HotStuffMessageHandler, HotStuffConfig, HotStuffMessage};

pub struct HotStuff<T, P>
where
    T: Transaction,
    P: TransactionPool<Transaction = T>,
{
    dispatcher: mpsc::Sender<HotStuffMessage>,
    handler: HotStuffMessageHandler,
    mempool: Arc<P>,
}

impl<T, P> HotStuff<T, P>
where
    T: Transaction,
    P: TransactionPool<Transaction = T>,
{
    pub fn new(config: HotStuffConfig, mempool: P) -> Self {
        let handler = HotStuffMessageHandler { to_hotstuff };

        Self {
            dispatcher,
            handler,
            mempool: Arc::new(mempool),
        }
    }

    pub async fn run(mut self) {
        while let Some(message) = self.mailbox.recv().await {
            match message {
                HotStuffMessage::Dummy {} => {
                    info!("Received dummy message");
                }
            }
        }
    }

    pub fn handler(&self) -> HotStuffMessageHandler {
        self.handler.clone()
    }

    pub fn mempool(&self) -> Arc<P> {
        self.mempool.clone()
    }
}
