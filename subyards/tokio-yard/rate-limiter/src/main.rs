use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;

#[derive(Debug)]
struct Resource {
    counter: usize,
    last_client: usize, // todo: add history of access?
}

async fn client(id: usize, resource: Arc<Mutex<Resource>>, sem: Arc<Semaphore>) {
    println!("[{id}] client start");

    // ask for semaphore permit
    let _permit = sem.acquire().await.unwrap();
    println!("[{id}] acquired permit");
    {
        let mut lock = resource.lock().unwrap();
        // modify resource
        lock.counter += 1;
        lock.last_client = id;
    } // lock get's out of scope here 
    // simulated delay
    let millis = rand::random_range(1..=10) * 500;
    let delay = std::time::Duration::from_millis(500 + millis);
    tokio::time::sleep(delay).await;
    println!("[{id}] client finish");
} // permit goes out of scope here

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_set = tokio::task::JoinSet::new();
    let data = Arc::new(Mutex::new(Resource {
        counter: 0,
        last_client: 0,
    }));

    // spawn 20 clients
    for i in 1..=20 {
        let cloned = data.clone();
        let cloned_sem = semaphore.clone();
        join_set.spawn(client(i, cloned, cloned_sem));
    }

    while let Some(res) = join_set.join_next().await {
        println!("{res:?}");
    }
    println!("Final data: {data:?}");
}
