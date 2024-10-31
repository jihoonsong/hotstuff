use hotstuff_crypto::{Aggregator, PublicKey, Signer};
use hotstuff_mempool::{Transaction, TransactionPoolExt};
use hotstuff_p2p::{Encodable, NetworkAction};
use std::sync::Arc;
use tokio::{
    sync::{mpsc, oneshot},
    time::{sleep, Duration},
};
use tracing::info;

use crate::{
    Block, Committee, HotStuffConfig, HotStuffMessage, HotStuffMessageHandler, LeaderElector,
    Round, Timeout,
};

pub struct HotStuff<T, P, L>
where
    T: Transaction,
    P: TransactionPoolExt<Transaction = T>,
    L: LeaderElector,
{
    from_hotstuff: mpsc::Receiver<HotStuffMessage<T>>,
    handler: HotStuffMessageHandler<T>,
    mempool: Arc<P>,
    to_network: Option<mpsc::Sender<NetworkAction>>,
    timeout: Timeout,
    round: Round,
    committee: Committee<L>,
    identity: PublicKey,
    signer: Signer,
    aggregator: Aggregator,
}

impl<T, P, L> HotStuff<T, P, L>
where
    T: Transaction,
    P: TransactionPoolExt<Transaction = T>,
    L: LeaderElector,
{
    pub fn new(
        config: HotStuffConfig,
        mempool: P,
        committee: Committee<L>,
        identity: PublicKey,
        signer: Signer,
        aggregator: Aggregator,
    ) -> Self {
        let (to_hotstuff, from_hotstuff) = mpsc::channel(config.mailbox_size);
        let handler = HotStuffMessageHandler { to_hotstuff };
        let timeout = Timeout::new(config.timeout);
        let round = Round::default();

        Self {
            from_hotstuff,
            handler,
            mempool: Arc::new(mempool),
            to_network: None,
            timeout,
            round,
            committee,
            identity,
            signer,
            aggregator,
        }
    }

    pub async fn run(mut self) {
        // Wait for the network to be ready.
        self.wait_for_network_ready().await;

        // Reset timer and propose a block if we are the leader.
        self.timeout.reset();
        if self.identity == self.committee.leader(self.round) {
            self.propose().await;
        }
        // Now we are guaranteed to make a progress.

        loop {
            tokio::select! {
                Some(message) = self.from_hotstuff.recv() => match message {
                    HotStuffMessage::Proposal(block) => {
                        self.handle_proposal(block).await;
                    },
                },
                () = &mut self.timeout => self.handle_timeout().await,
            }
        }
    }

    pub fn handler(&self) -> HotStuffMessageHandler<T> {
        self.handler.clone()
    }

    pub fn mempool(&self) -> Arc<P> {
        self.mempool.clone()
    }

    pub fn set_network(&mut self, network: mpsc::Sender<NetworkAction>) {
        self.to_network = Some(network);
    }

    async fn wait_for_network_ready(&self) {
        loop {
            let (reply, response) = oneshot::channel();

            self.to_network
                .as_ref()
                .unwrap()
                .send(NetworkAction::IsReady { reply })
                .await
                .unwrap();

            if response.await.unwrap() {
                info!("{}: Network is ready", self.identity);
                break;
            }

            info!("{}: Network is not ready yet", self.identity);
            sleep(Duration::from_secs(1)).await;
        }
    }

    async fn propose(&mut self) {
        let block = Block::new(
            self.identity.clone(),
            self.round,
            self.mempool.pending_transactions().await,
        );

        self.to_network
            .as_mut()
            .unwrap()
            .send(NetworkAction::Broadcast {
                message: HotStuffMessage::Proposal(block).encode(),
            })
            .await
            .unwrap();
    }

    async fn handle_proposal(&self, block: Block<T>) {
        info!("{}: Received a proposal {:?}", self.identity, block);
    }

    async fn handle_timeout(&self) {}
}
