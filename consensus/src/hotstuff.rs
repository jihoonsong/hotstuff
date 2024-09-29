use tokio::sync::mpsc;
use tracing::info;

use crate::{HotStuffConfig, HotStuffMessage};

pub struct HotStuff {
    dispatcher: mpsc::Sender<HotStuffMessage>,
    mailbox: mpsc::Receiver<HotStuffMessage>,
}

impl HotStuff {
    pub fn new(config: HotStuffConfig) -> Self {
        let (dispatcher, mailbox) = mpsc::channel(config.mailbox_size);

        Self {
            dispatcher,
            mailbox,
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
}
