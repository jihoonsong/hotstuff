use hotstuff_crypto::{Aggregator, PublicKey, Signer};
use hotstuff_mempool::{Transaction, TransactionPoolExt};
use hotstuff_p2p::NetworkAction;
use hotstuff_primitives::{Encodable, Round};
use std::sync::Arc;
use tokio::{
    sync::{mpsc, oneshot},
    time::{sleep, Duration},
};
use tracing::info;

use crate::{
    Committee, HotStuffConfig, HotStuffMessage, HotStuffMessageHandler, LeaderElector, Proposer,
    SignedBlock, Timeout,
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
    proposer: Proposer<T, P>,
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
        let mempool = Arc::new(mempool);
        let timeout = Timeout::new(config.timeout);
        let round = Round::default();
        let proposer = Proposer::new(identity.clone(), mempool.clone(), signer.clone());

        Self {
            from_hotstuff,
            handler,
            mempool,
            to_network: None,
            timeout,
            round,
            committee,
            identity,
            proposer,
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
        let signed_block = self.proposer.propose(self.round).await;

        self.to_network
            .as_mut()
            .unwrap()
            .send(NetworkAction::Broadcast {
                message: HotStuffMessage::Proposal(signed_block).encode(),
            })
            .await
            .unwrap();
    }

    async fn handle_proposal(&self, block: SignedBlock<T>) {
        info!("{}: Received a proposal {:?}", self.identity, block);
    }

    async fn handle_timeout(&self) {}
}
