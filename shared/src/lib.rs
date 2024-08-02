use mini_redis::Frame;
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
    pub fn to_frame(&self) -> Frame {
        Frame::Bulk(bincode::serialize(&self).unwrap().into())
        
    }

    pub fn from_frame(frame: &Frame) -> Self {
        match frame {
            Frame::Bulk(ref bytes) => bincode::deserialize(bytes).unwrap(),
            _ => unreachable!(), 
        }
    }
}