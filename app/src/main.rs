mod client;
mod config;
mod network;

use crate::client::Client;
use crate::config::Config;
use crate::network::Network;

use std::{fs, io};
use toml;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Init tracing.
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(io::stdout)
        .init();

    // Run a client and a network of nodes.
    run().await;
}

async fn run() {
    // Read config.
    let config_content = fs::read_to_string("./Config.toml").expect("Failed to read config");
    let config: Config = toml::from_str(&config_content).expect("Failed to parse config");

    // Start client.
    let client = Client::new(config.client);
    let mut client_task = tokio::spawn(client.run());

    // Start network.
    let network: Network = Network::new(config.nodes);
    let mut network_task = tokio::spawn(network.run());

    match tokio::try_join!(&mut client_task, &mut network_task) {
        Ok(_) => {
            info!("All tasks completed");
        }
        Err(e) => {
            info!(error=?e, "An error occured while running tasks");
            client_task.abort();
            network_task.abort();
        }
    }
}
