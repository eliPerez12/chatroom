use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageLog {
    pub username: String,
    pub message: String,
} 


#[derive(Serialize, Deserialize, Debug)]
pub struct MessageLogs {
    pub messages: Vec<MessageLog>,
}
