use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc};

#[derive(Debug)]
struct ChatRoom {
    tx: broadcast::Sender<String>,
    rx: mpsc::Receiver<String>,
}

impl ChatRoom {
    fn new() -> (Self, mpsc::Sender<String>, broadcast::Sender<String>) {
        let (tx, rx) = mpsc::channel::<String>(10);
        let (btx, _) = broadcast::channel::<String>(10);
        (
            ChatRoom {
                tx: btx.clone(),
                rx: rx,
            },
            tx,
            btx,
        )
    }

    async fn run(mut self) {
        println!("Waitig for messages in the room");
        while let Some(msg) = self.rx.recv().await {
            println!("server got msg: {msg}");
            let _ = self.tx.send(msg);
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Opened TCP server connection at localhost:7575");
    let listener = TcpListener::bind("localhost:7575").await.unwrap();
    let (chat_room, tx, btx) = ChatRoom::new();
    tokio::spawn(chat_room.run());

    // each connection is new tokio async task
    loop {
        let (tcp_stream, socket_addr) = listener.accept().await.unwrap();
        let client_tx = tx.clone();
        let mut clinet_rx = btx.subscribe();

        tokio::spawn(async move {
            println!("Connected to socket {socket_addr:?}");

            let (reader, mut writer) = tcp_stream.into_split();
            let mut lines = BufReader::new(reader).lines();

            // TODO add a join-banner - do you want to join channel?
            // if yes, send a welcome message, if no bye message and close connection

            println!("Sending Join-banner to {socket_addr}");
            let banner_msg = "Join chat-room? (y for yes, anything for no)";
            let _ = writer.write_all(banner_msg.as_bytes()).await;

            // join answer
            match lines.next_line().await {
                Ok(Some(answer)) => {
                    println!("[{socket_addr}] Got join banner reply: {answer}");
                    if answer == "y" {
                        println!("[{socket_addr}] Sending welcome msg");
                        let welcom_msg =
                            "===============\nWelcome to the chat room!\n===============\n";
                        let _ = writer.write_all(welcom_msg.as_bytes()).await;
                    } else {
                        println!("[{socket_addr}] Sending bye msg and closing connection");
                        let _ = writer.write_all(b"Bye!\n").await;
                        return;
                    }
                }
                Ok(None) => {
                    println!("[{socket_addr}] Connection lost");
                    return;
                }
                Err(e) => {
                    println!("[{socket_addr}] Connection error {e:?}");
                    return;
                }
            }

            loop {
                tokio::select! {
                    // from tcp client
                    result = lines.next_line() => {
                        match result {
                            Ok(Some(msg)) => {
                                println!("got line: {msg}");
                                client_tx.send(format!("{socket_addr}:{msg}")).await.unwrap();
                            }
                            Ok(None) => {
                                println!("Conn lost {socket_addr}");
                                break;
                            }
                            Err(e) => {
                                println!("conn err {e:?}");
                                break;
                            }
                        }
                    }

                    // from broadcast
                    result = clinet_rx.recv() => {
                        if let Ok(msg) = result {
                            if !msg.as_str().contains(format!("{socket_addr}").as_str()){
                                let _ = writer.write_all(format!("Broadcast: {msg}\n").as_bytes()).await;
                            }
                        }
                    }
                }
            }
        });
    }
}
