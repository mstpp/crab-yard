use std::time::Duration;
use tokio::sync::{broadcast, mpsc};

const MAX_CHATTERS: usize = 10;

// -------------------------------------------------
// 1. THE SERVER ACTOR
// -------------------------------------------------
struct ChatRoom {
    // The server's inbox (from clients)
    rx: mpsc::Receiver<String>,
    // The server's outbox (to clients)
    btx: broadcast::Sender<String>,
}

impl ChatRoom {
    /// Creates the chat room and returns the channels needed to talk to it
    fn new() -> (Self, mpsc::Sender<String>, broadcast::Sender<String>) {
        let (tx, rx) = mpsc::channel(MAX_CHATTERS);
        let (btx, _) = broadcast::channel(MAX_CHATTERS);

        let server_actor = Self {
            rx,
            btx: btx.clone(),
        };
        (server_actor, tx, btx)
    }

    /// The infinite loop that drives the server
    async fn run(mut self) {
        println!("[Server] Started and listening for messages...");
        // This loop processes one message at a time, completely avoiding data races.
        while let Some(msg) = self.rx.recv().await {
            println!("[Server] Routing message: {:?}", msg);
            // Broadcast it to everyone. Ignore errors if no one is listening yet.
            let _ = self.btx.send(msg);
        }
        println!("[Server] Shutting down. All senders dropped.");
    }
}

// -------------------------------------------------
// 2. THE CLIENT ACTOR
// -------------------------------------------------
struct Chatter {
    id: usize,
    // The client's outbox (to the server)
    server_tx: Option<mpsc::Sender<String>>, // wrap sender in option to drop early
    // The client's inbox (from the server's broadcast)
    room_rx: broadcast::Receiver<String>,
}

impl Chatter {
    /// The client loop that handles sending AND receiving concurrently
    async fn run(mut self) {
        // We will simulate sending 2 messages, then stop sending.
        let mut messages_to_send = 2;
        let mut interval = tokio::time::interval(Duration::from_millis(500));

        loop {
            // tokio::select! waits on multiple async branches concurrently.
            // Whichever finishes first gets executed.
            tokio::select! {

                // Branch A: We received a message from the server
                res = self.room_rx.recv() => {
                    match res {
                        Ok(msg) => println!(" -> Chatter {} read: {:?}", self.id, msg),
                        Err(_) => {
                            println!("Chatter {} disconnected (server closed).", self.id);
                            break;
                        }
                    }
                }

                // Branch B: Only tick if we still have messages to send
                _ = interval.tick(), if messages_to_send > 0 => {
                    let msg = format!("Hello from C{}", self.id);

                    // Send the message using the Option
                    if let Some(tx) = &self.server_tx {
                        let _ = tx.send(msg).await;
                    }

                    messages_to_send -= 1;

                    // If we are done sending, drop our transmitter!
                    if messages_to_send == 0 {
                        self.server_tx = None; // This drops the mpsc::Sender
                        println!("Chatter {} finished typing and dropped its tx.", self.id);
                    }
                }
            }
        }
    }
}

// -------------------------------------------------
// 3. WIRING IT TOGETHER
// -------------------------------------------------
#[tokio::main]
async fn main() {
    // 1. Initialize the channels and the server actor
    let (chatroom, server_tx, server_btx) = ChatRoom::new();

    // 2. Spawn the server in the background
    let server_handle = tokio::spawn(chatroom.run());

    // 3. Spawn 3 isolated client actors
    let mut client_handles = vec![];
    for id in 1..=3 {
        let client = Chatter {
            id,
            server_tx: Some(server_tx.clone()),
            room_rx: server_btx.subscribe(),
        };
        client_handles.push(tokio::spawn(client.run()));
    }

    // 4. Drop the main thread's copy of the Sender.
    // If we don't do this, the server's `while let Some(msg) = self.rx.recv()`
    // will wait forever because it thinks `main` might still send something.
    drop(server_tx);
    drop(server_btx);

    // Wait for the clients and server to finish their work
    for handle in client_handles {
        let _ = handle.await;
    }
    let _ = server_handle.await;

    println!("Simulation complete.");
}
