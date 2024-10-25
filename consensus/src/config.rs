use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HotStuffConfig {
    pub mailbox_size: usize,
    pub timeout: u64,
}
