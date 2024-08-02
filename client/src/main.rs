use mini_redis::{Connection, Frame};
use tokio::net::TcpStream;
use shared::ClientMessage;

mod gui;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:6379").await?;
    let mut connection = Connection::new(stream);
    let message = ClientMessage::Message {
        username: "Eli".to_string(),
        message: "Hello from client!".to_string(),
    };

    loop {
        connection.write_frame(&message.to_frame()).await?;
        if let Some(frame) = connection.read_frame().await.unwrap() {
            println!("GOT: {:?}", ClientMessage::from_frame(&frame));
        } else {
            break
        }
    }
    println!("Lost connection to server.");
    // let options = eframe::NativeOptions::default();
    // eframe::run_native(
    //     "Chat Room",
    //     options,
    //     Box::new(|_cc| Ok(Box::new(ChatApp::default()))),
    // )
    // .unwrap();
    Ok(())
}
