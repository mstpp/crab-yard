use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[tokio::main]
async fn main() {
    let start = std::time::SystemTime::now();
    let counter = Arc::new(AtomicUsize::new(0));

    let mut t_set = tokio::task::JoinSet::new();

    for _ in 1..=5000 {
        let clone = counter.clone();
        t_set.spawn(async move {
            clone.fetch_add(1, Ordering::Relaxed);
        });
    }

    while let Some(res) = t_set.join_next().await {
        res.unwrap();
    }

    let elapsed = start.elapsed().unwrap();
    println!("Counter: {counter:?}. Elapsed {elapsed:?}");
}
