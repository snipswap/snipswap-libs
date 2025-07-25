use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    pub content_cid: String,
    pub sender: String,
    pub recipient: String,
    pub nonce: Vec<u8>,
}
