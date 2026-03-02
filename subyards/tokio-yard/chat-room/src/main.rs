// TODO implement timeouts
// TODO implement logs redirection instead of stdout
use bytes::Bytes;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{self, Receiver, Sender};

#[derive(Debug, Clone)]
struct ChatMessage {
    sender: SocketAddr,
    msg: Bytes, // efficient zero copy Arc behind
                // msg: Arc<[u8]>, // double allocation when Arc::from(format!())
}

struct ChatterGuard(Arc<AtomicUsize>);
impl Drop for ChatterGuard {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
        println!(
            "A chatter left. Remaining: {}", // TODO broadcast
            self.0.load(Ordering::SeqCst)
        );
    }
}

async fn handle_connection(
    tcp_stream: TcpStream,
    socket_addr: SocketAddr,
    bcast_tx: Sender<ChatMessage>,
    mut bcast_rx: Receiver<ChatMessage>,
    online_counter: Arc<AtomicUsize>,
) {
    println!("Connected to socket {socket_addr:?}");
    let (reader, mut writer) = tcp_stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    println!("Sending Join-banner to {socket_addr}");
    let banner_msg = "Join chat-room? (y for yes, anything for no)";
    let _ = writer.write_all(banner_msg.as_bytes()).await;
    let _guard = ChatterGuard(online_counter.clone());
    online_counter.fetch_add(1, Ordering::SeqCst);

    // join-banner
    match lines.next_line().await {
        Ok(Some(answer)) => {
            println!("[{socket_addr}] Got join banner reply: {answer}");
            if answer == "y" {
                println!("[{socket_addr}] Sending welcome msg");
                let welcome_msg = format!(
                    r#"===============
Welcome to the chat room!
===============
{} chatters online
"#,
                    online_counter.load(Ordering::SeqCst)
                );
                let _ = writer.write_all(welcome_msg.as_bytes()).await;
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
                        let _ = bcast_tx.send(chat_msg);
                    }
                    Ok(None) | Err(_) => {
                        println!("[{socket_addr}] Connection closed or error. Disconnecting.");
                        break;
                    }
                }
            }

            // from broadcast
            result = bcast_rx.recv() => {
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
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    println!("Opened TCP server connection at localhost:7575");
    let listener = TcpListener::bind("localhost:7575").await?;
    let (bcast_tx, _) = broadcast::channel::<ChatMessage>(10);
    let chatters = Arc::new(AtomicUsize::new(0));

    loop {
        let (tcp_stream, socket_addr) = match listener.accept().await {
            Ok((stream, addr)) => (stream, addr),
            Err(e) => {
                println!("Accipting socket connection error: {e}");
                tokio::time::sleep(Duration::from_millis(500)).await;
                continue;
            }
        };

        tokio::spawn(handle_connection(
            tcp_stream,
            socket_addr,
            bcast_tx.clone(),
            bcast_tx.subscribe(),
            chatters.clone(),
        ));
    }
}
