use std::time::Duration;

use tokio::sync::{broadcast, mpsc};

static MAX_CHATTERS: usize = 10;
static NUM_CHATTERS: usize = 3;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(MAX_CHATTERS);
    let (btx, _) = broadcast::channel::<String>(MAX_CHATTERS);

    for i in 1..=NUM_CHATTERS {
        let tx = tx.clone();
        let mut brx = btx.subscribe();

        tokio::spawn(async move {
            // why sleep?
            // w/o sleep, drop Sender will happen too fast,
            // so that rx channel will be already closed,
            // before broadcast receive could happen
            tokio::time::sleep(Duration::from_millis(500)).await;

            // only one clinet send msg to broadcast channel
            if i == 1 {
                println!("send msg from client 1");
                tx.send(format!("hello from c{i}")).await.unwrap();
            }
            drop(tx);

            // get broadcast message
            while let Ok(msg) = brx.recv().await {
                println!("chatter c{i} got: {msg:?}");
            }
        });
    }

    drop(tx);

    while let Some(msg) = rx.recv().await {
        println!("server got msg: {msg:?}");
        btx.send(msg).unwrap();
    }

    drop(btx);
}
