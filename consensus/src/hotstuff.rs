use hotstuff_mempool::{Transaction, TransactionPoolExt};
use hotstuff_p2p::{Encodable, NetworkAction};
use std::{net::SocketAddr, sync::Arc, thread, time::Duration};
use tokio::sync::{mpsc, oneshot};
use tracing::info;

use crate::{
    Block, HotStuffConfig, HotStuffMessage, HotStuffMessageHandler, LeaderElector, Round, Timeout,
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
    leader_elector: L,
    identity: SocketAddr,
    round: Round,
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
        leader_elector: L,
        identity: SocketAddr,
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
            leader_elector,
            identity,
            round,
        }
    }

    pub async fn run(mut self) {
        // Wait for the network to be ready.
        self.wait_for_network_ready().await;

        // Reset timer and propose a block if we are the leader.
        self.timeout.reset();
        if self.identity == self.leader_elector.leader(self.round) {
            self.propose().await;
        }
        // Now we are guaranteed to make a progress.

        loop {
            tokio::select! {
                Some(message) = self.from_hotstuff.recv() => match message {
                    HotStuffMessage::Proposal(block) => {
                        info!("Received a proposal {:?}", block);
                    },
                },
                () = &mut self.timeout => self.timeout().await,
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
            let (respond, response) = oneshot::channel();

            self.to_network
                .as_ref()
                .unwrap()
                .send(NetworkAction::IsReady { respond })
                .await
                .unwrap();

            if response.await.unwrap() {
                info!("Network is ready");
                break;
            }

            info!("Network is not ready yet");
            thread::sleep(Duration::from_secs(1));
        }
    }

    async fn propose(&mut self) {
        let block = Block::new(
            self.identity,
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

    async fn timeout(&self) {
        info!("timeout!");
    }
}
