use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    time::Duration,
    vec,
};

use gui::ChatApp;
use shared::*;

pub fn server_connection(
    user_message: Arc<Mutex<Option<MessageLog>>>,
    messages: Arc<Mutex<Vec<MessageLog>>>,
    last_message_recived: Arc<Mutex<u32>>,
) {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:6379") {
        println!("Connected to server.");
        let mut buffer = [0; 8192];
        loop {
            let mut lock = user_message.lock().unwrap();
            let user_message = lock.clone();
            let bytes = bincode::serialize(&(user_message, *last_message_recived.lock().unwrap())).unwrap();
            if stream.write_all(&bytes).is_err() {
                break;
            }
            *lock = None;
            if stream.read(&mut buffer).is_err() {
                break;
            }
            let logs: MessageLogs = bincode::deserialize(&buffer).unwrap();
            let mut lock = messages.lock().unwrap();
            *lock = vec![];
            for log in logs.messages {
                lock.push(log)
            }
        }
    }
}

mod gui;

fn main() {
    let user_message: Arc<Mutex<Option<MessageLog>>> = Arc::new(Mutex::new(None));
    let messages: Arc<Mutex<Vec<MessageLog>>> = Arc::new(Mutex::new(vec![]));
    let last_message_recived = Arc::new(Mutex::new(0));

    let user_message_clone = user_message.clone();
    let messages_clone = messages.clone();
    let last_message_recived_clone = Arc::new(Mutex::new(0));
    std::thread::spawn(move || loop {
        server_connection(
            user_message_clone.clone(),
            messages_clone.clone(),
            last_message_recived_clone.clone(),
        );
        println!("Attempting to connect to server...");
        std::thread::sleep(Duration::from_secs(3));
    });

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Chat Room",
        options,
        Box::new(|_cc| {
            Ok(Box::new(ChatApp::new(
                user_message,
                messages,
                last_message_recived,
            )))
        }),
    )
    .unwrap();
}
