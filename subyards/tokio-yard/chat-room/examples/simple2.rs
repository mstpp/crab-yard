use tokio::sync::broadcast::Receiver as BroadcastRx;
use tokio::sync::mpsc::Sender as MpscTx;

const MAX_CHATTERS: usize = 10;

#[derive(Debug)]
struct Chatter {
    tx: MpscTx<String>,
    rx: BroadcastRx<String>,
}

#[tokio::main]
async fn main() {
    println!("starting a chat room");
    let (tx, mut rx) = tokio::sync::mpsc::channel(MAX_CHATTERS);
    // We don't need the original broadcast receiver, subscribers will create their own
    let (btx, _) = tokio::sync::broadcast::channel(MAX_CHATTERS);

    let mut chatters: Vec<Chatter> = vec![];

    // create 3 chatters
    for _ in 1..=3 {
        let c = Chatter {
            tx: tx.clone(),
            rx: btx.subscribe(),
        };
        chatters.push(c);
    }

    // Drop the original tx so the mpsc channel can actually close later
    // we drop it after all clients cloned their Sender
    drop(tx);

    // Clone only the specific sender we need for the first task
    let c1_tx = chatters[0].tx.clone();
    tokio::spawn(async move {
        println!("c1 send a msg");
        c1_tx.send("hello from c1".to_string()).await.unwrap();
    });

    // Bind the handle so we can await it, and use `mut c` to allow recv()
    let verify_handle = tokio::spawn(async move {
        for mut c in chatters {
            let got = c.rx.recv().await.unwrap();
            // Note: BroadcastRx doesn't implement Debug, so we just print the tx and the message
            println!("chatter {:?} got: {:?}", c.tx, got);
        }
    });

    // Server loop: routes incoming mpsc messages to the broadcast channel
    while let Some(msg) = rx.recv().await {
        println!("server got msg: {msg:?}");
        // We ignore the error in case there are no active subscribers yet
        let _ = btx.send(msg);
    }

    // Prevent main from exiting before verification finishes
    verify_handle.await.unwrap();
    println!("Server shutting down safely.");
}
