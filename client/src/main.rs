use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    time::Duration,
    vec,
};

use gui::ChatApp;
use shared::ClientMessage;

pub fn server_connection(
    user_message: Arc<Mutex<ClientMessage>>,
    messages: Arc<Mutex<Vec<ClientMessage>>>,
) {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:6379") {
        println!("Connected to server.");
        let mut buffer = [0; 1024];
        loop {
            let mut lock = user_message.lock().unwrap();
            let bytes = lock.serialize();
            if stream.write_all(&bytes).is_err() {
                break;
            }
            *lock = ClientMessage::None;
            if stream.read(&mut buffer).is_err() {
                break;
            }
            let message = ClientMessage::deserialize(&buffer);
            match message {
                ClientMessage::None{..} => (),
                ClientMessage::Message { .. } => messages.lock().unwrap().push(message),
            }
        }
    }
}

mod gui;

fn main() {
    let user_message: Arc<Mutex<ClientMessage>> = Arc::new(Mutex::new(ClientMessage::None));
    let messages: Arc<Mutex<Vec<ClientMessage>>> = Arc::new(Mutex::new(vec![]));

    let user_message_clone = user_message.clone();
    let messages_clone = messages.clone();
    std::thread::spawn(move || loop {
        server_connection(user_message_clone.clone(), messages_clone.clone());
        println!("Attempting to connect to server...");
        std::thread::sleep(Duration::from_secs(3));
    });

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Chat Room",
        options,
        Box::new(|_cc| Ok(Box::new(ChatApp::new(user_message, messages)))),
    )
    .unwrap();
}
