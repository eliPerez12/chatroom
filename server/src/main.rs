use std::sync::Arc;

use shared::*;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, sync::Mutex};

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
    let mut buffer = [0; 2048];

    loop {
        // Recive message from client
        let n = match socket.read(&mut buffer).await {
            Ok(0) => return, 
            Ok(n) => n,
            Err(_) => return,
        };
        let client_message: (Option<MessageLog>, u32) = bincode::deserialize(&buffer).unwrap();

        // Update log
        if let Some(message_log) = client_message.0 {
            println!("{}: {}", &message_log.username, &message_log.message);
            logs.lock().await.messages.push(message_log);
        }

        // Send updates to client
        let logs = logs.lock().await;
        if socket.write_all(&bincode::serialize(&*logs).unwrap()).await.is_err() {
            return;
        }
    }
}
