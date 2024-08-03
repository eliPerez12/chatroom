use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    None,
    Message {
        username: String,
        message: String,
    }
}



impl ClientMessage {
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
        
    }

    pub fn deserialize(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }
}