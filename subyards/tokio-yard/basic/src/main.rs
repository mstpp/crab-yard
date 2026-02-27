use std::time::Duration;

use tokio::task::JoinSet;

async fn process(id: usize) -> usize {
    println!("[{id}] Processing start");
    let milis = rand::random_range(100..=500);
    let duration = tokio::time::Duration::from_millis(milis);
    tokio::time::sleep(duration).await;
    println!("[{id}] Processing finished");
    id
}

#[tokio::main]
async fn main() {
    // single handle - JoinHandle
    let handle = tokio::spawn(process(111));
    // let res = handle.await;
    // println!("JoinHandle result: {res:?}");
    // println!("========================");
    //
    // // multi-handle, JoinSet
    // let mut set = JoinSet::new();
    // for i in 1..=5 {
    //     set.spawn(process(i));
    // }
    //
    // while let Some(res) = set.join_next().await {
    //     println!("Task finished, result: {res:?}");
    // }
    tokio::time::sleep(Duration::from_millis(1000)).await;
}
