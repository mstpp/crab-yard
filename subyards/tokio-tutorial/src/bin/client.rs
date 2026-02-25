use std::time::Duration;

use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;

static CHANNEL_SIZE: usize = 5;

#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}

async fn client_send(tx: Sender<Command>, k: String, v: Bytes) {
    // add reandom delay
    let millis = rand::random_range(1..=20);
    sleep(Duration::from_millis(500 * millis)).await;
    // now send
    if let Err(_) = tx.send(Command::Set { key: k, val: v }).await {
        println!("receiver dropped");
        return;
    }
}
#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Command>(CHANNEL_SIZE);

    // msg clients
    for i in 1..=10 {
        tokio::spawn(client_send(
            tx.clone(),
            format!("foo-{i}"),
            Bytes::from("bar"),
        ));
    }
    drop(tx);

    // msg server - redis client connetion manager
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();
    while let Some(i) = rx.recv().await {
        println!("got from msg client = {:?}", &i);

        match i {
            Command::Set { key: k, val: v } => {
                client.set(&k, v).await.unwrap();
                // println!("set {}", &k);
            }
            Command::Get { key: k } => {
                let mut res = Bytes::new();
                if let Some(v) = client.get(&k).await.unwrap() {
                    res = v;
                } else {
                    res = "empty".into()
                };
                let res_str = str::from_utf8(&res).unwrap();
                println!("Got value: {res_str}");
            }
        }
    }

    // validate
    for i in 1..=10 {
        let res = client.get(format!("foo-{i}").as_str()).await.unwrap();
        assert_eq!(res, Some(Bytes::from("bar")))
    }
}
