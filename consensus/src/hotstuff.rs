use hotstuff_mempool::{Transaction, TransactionPool};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;

use crate::{HotStuffConfig, HotStuffMessage};

pub struct HotStuff<T, P>
where
    T: Transaction,
    P: TransactionPool<Transaction = T>,
{
    dispatcher: mpsc::Sender<HotStuffMessage>,
    mailbox: mpsc::Receiver<HotStuffMessage>,
    mempool: Arc<P>,
}

impl<T, P> HotStuff<T, P>
where
    T: Transaction,
    P: TransactionPool<Transaction = T>,
{
    pub fn new(config: HotStuffConfig, mempool: P) -> Self {
        let (dispatcher, mailbox) = mpsc::channel(config.mailbox_size);

        Self {
            dispatcher,
            mailbox,
            mempool: Arc::new(mempool),
        }
    }

    pub async fn run(mut self) {
        while let Some(message) = self.mailbox.recv().await {
            match message {
                HotStuffMessage::Dummy { data } => {
                    info!("Received dummy message: {:?}", data);
                }
            }
        }
    }

    pub fn mailbox(&self) -> mpsc::Sender<HotStuffMessage> {
        self.dispatcher.clone()
    }

    pub fn mempool(&self) -> Arc<P> {
        self.mempool.clone()
    }
}
