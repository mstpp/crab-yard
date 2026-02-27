use bytes::Bytes;
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
struct ChatMessage {
    sender: SocketAddr,
    msg: Bytes, // efficient zero copy Arc behind
                // msg: Arc<[u8]>, // double allocation when Arc::from(format!())
}

#[tokio::main]
async fn main() {
    println!("Opened TCP server connection at localhost:7575");
    let listener = TcpListener::bind("localhost:7575").await.unwrap();
    let (bcast_tx, _) = broadcast::channel::<ChatMessage>(10);

    loop {
        let (tcp_stream, socket_addr) = listener.accept().await.unwrap();
        let client_tx = bcast_tx.clone();
        let mut clinet_rx = bcast_tx.subscribe();

        tokio::spawn(async move {
            println!("Connected to socket {socket_addr:?}");

            let (reader, mut writer) = tcp_stream.into_split();
            let mut lines = BufReader::new(reader).lines();

            println!("Sending Join-banner to {socket_addr}");
            let banner_msg = "Join chat-room? (y for yes, anything for no)";
            let _ = writer.write_all(banner_msg.as_bytes()).await;

            // join-banner
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
                                let bcast_msg = Bytes::from(format!("[{}]: {}\n", socket_addr, msg));
                                let chat_msg = ChatMessage {sender: socket_addr, msg: bcast_msg};
                                let _ = client_tx.send(chat_msg);
                            }
                            Ok(None) | Err(_) => {
                                println!("[{socket_addr}] Connection closed or error. Disconnecting.");
                                break;
                            }
                        }
                    }

                    // from broadcast
                    result = clinet_rx.recv() => {
                        match result {
                            Ok(msg) =>
                                if msg.sender != socket_addr {
                                    if writer.write_all(&msg.msg).await.is_err() {
                                        println!("[{socket_addr}] Write failed (client disconnected). Dropping task.");
                                        break;
                                    }
                                },
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                                // Tokio's broadcast handles backpressure by dropping messages for slow readers
                                println!("[{socket_addr}] Client lagged, missed {skipped} messages");
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                                println!("Channel closed");
                                break;
                            }
                        }
                    }
                }
            }
        });
    }
}
