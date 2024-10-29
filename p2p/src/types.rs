use futures::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use tokio_util::{
    bytes::Bytes,
    codec::{Framed, LengthDelimitedCodec},
};

pub(crate) type Reader = SplitStream<Framed<TcpStream, LengthDelimitedCodec>>;
pub(crate) type Writer = SplitSink<Framed<TcpStream, LengthDelimitedCodec>, Bytes>;
