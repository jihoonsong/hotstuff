use futures::future::join_all;
use hotstuff_rpc::Transaction;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::ws_client::WsClientBuilder;
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    sync::Mutex,
    time::{sleep, Duration},
};
use tracing::{debug, info};

pub struct Client {
    nodes: Vec<SocketAddr>,
    nonce: Arc<Mutex<u128>>,
}

impl Client {
    pub fn new(nodes: Vec<SocketAddr>) -> Self {
        Self {
            nodes,
            nonce: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn run(self) {
        join_all(self.nodes.into_iter().enumerate().map(|(index, node)| {
            tokio::spawn(Self::send_transactions(
                format!("ws://{}", node),
                format!("transaction to node{}", index),
                self.nonce.clone(),
            ))
        }))
        .await;
    }

    async fn send_transactions(url: String, data: String, nonce: Arc<Mutex<u128>>) {
        let client = WsClientBuilder::default()
            .build(&url)
            .await
            .expect("Failed to build WsClientBuilder");

        loop {
            // Set transaction data. Nonce is shared across all transactions.
            let mut nonce = nonce.lock().await;
            let transaction = Transaction {
                nonce: *nonce,
                data: data.clone(),
            };
            *nonce += 1;
            drop(nonce);
            info!("client sends transaction: {:?}", transaction);

            let response: String = loop {
                match client
                    .request("transaction_send", rpc_params![transaction.clone()])
                    .await
                {
                    Ok(response) => break response,
                    Err(e) => {
                        debug!("Failed to send transaction to node at {url}: {e}. Retrying...");
                        sleep(Duration::from_millis(100)).await;
                    }
                };
            };
            info!("client received response: {}", response);
        }
    }
}
