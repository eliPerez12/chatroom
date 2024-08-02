use shared::ClientMessage;
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

struct ClientMessages {
    messages: Vec<String>,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening for connection.");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    while let Ok(frame) = connection.read_frame().await {
        if let Some(frame) = frame {
            println!("GOT: {:?}", ClientMessage::from_frame(&frame));
            connection.write_frame(&frame).await.unwrap();
        }
    }
    println!("Lost connection to client.");
}