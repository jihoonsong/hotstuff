use async_trait::async_trait;
use futures::stream::SplitSink;
use tokio::net::TcpStream;
use tokio_util::bytes::Bytes;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

pub type Writer = SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>;

#[async_trait]
pub trait MessageHandler: Clone + Send + Sync + 'static {
    async fn dispatch(&self, writer: &mut Writer, message: Bytes);
}
