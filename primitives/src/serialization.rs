use tokio_util::bytes::Bytes;

pub trait Encodable {
    fn encode(self) -> Bytes;
}

pub trait Decodable {
    fn decode(data: Bytes) -> Self;
}
