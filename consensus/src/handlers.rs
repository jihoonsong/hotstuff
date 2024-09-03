use async_trait::async_trait;
use futures_util::sink::SinkExt;
use hotstuff_p2p::{MessageHandler, Writer};
use tokio::sync::mpsc;
use tokio_util::bytes::Bytes;
use tracing::debug;

#[derive(Debug)]
pub enum HotStuffMessage {}

#[derive(Clone)]
pub struct HotStuffHandler {
    pub sender: mpsc::Sender<HotStuffMessage>,
}

#[async_trait]
impl MessageHandler for HotStuffHandler {
    async fn dispatch(&self, writer: &mut Writer, message: Bytes) {
        debug!("Handler received {:?}", message);

        let _ = writer.send(Bytes::from("Reply from handler")).await;
    }
}
