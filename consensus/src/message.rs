use tokio_util::bytes::Bytes;

pub enum HotStuffMessage {
    Dummy { data: Bytes },
}
