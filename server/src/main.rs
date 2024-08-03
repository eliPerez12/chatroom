use std::sync::{Arc, Mutex};

use shared::ClientMessage;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

pub struct MessageLog {
    username: String,
    message: String,
} 

impl MessageLog {
    pub fn from_client_message(message: ClientMessage) -> Option<Self> {
        match message {
            ClientMessage::None{..} => None,
            ClientMessage::Message { username, message } => Some(MessageLog {
                username,
                message,
            }),
        }
    }

    pub fn to_client_message(&self) -> ClientMessage {
        ClientMessage::Message {
            username: self.username.clone(),
            message: self.message.clone(),
        }
    }
}

pub struct MessageLogs {
    messages: Vec<MessageLog>,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let logs = Arc::new(Mutex::new(MessageLogs {
        messages: vec![],
    }));
    println!("Listening for connection.");

    loop {
        let logs_clone = logs.clone();
        let (socket, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket, logs_clone).await;
            println!("{addr} closed connection.");
        });
    }
}

async fn process(mut socket: TcpStream, logs: Arc<Mutex<MessageLogs>>) {
    let mut buffer = [0; 1024];

    loop {
        let n = match socket.read(&mut buffer).await {
            Ok(0) => return, 
            Ok(n) => n,
            Err(_) => return,
        };
        let client_message = ClientMessage::deserialize(&buffer);
        
        if socket.write_all(&buffer[0..n]).await.is_err() {
            // Handle write errors
            return;
        }
        
        if let Some(message_log) = MessageLog::from_client_message(client_message) {
            println!("{}: {}", &message_log.username, &message_log.message);
            logs.lock().unwrap().messages.push(message_log);
        }
    }
 
}
