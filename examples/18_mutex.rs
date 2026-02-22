use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

fn process(id: usize, counter: Arc<Mutex<usize>>, tx: mpsc::Sender<usize>) {
    // println!("[{id}] Process start");
    let mut c = counter.lock().unwrap(); // panic if poisoned, err on other thread
    *c += 1;
    tx.send(id).unwrap();
}

fn main() {
    let threads_num = 5000;
    let start = SystemTime::now();

    let (tx, rx) = mpsc::channel();
    let counter = Arc::new(Mutex::new(0_usize));

    for i in 1..=threads_num {
        let tx_clone = tx.clone(); // Each thread needs its own "sender"
        let counter_clone = counter.clone();
        std::thread::spawn(move || process(i, counter_clone, tx_clone));
    }

    // Drop the original sender so the receiver knows no more messages are coming
    drop(tx);

    // This loop executes as soon as ANY thread sends a message
    // for finished_id in rx {
    //     println!("Received notification: Process {finished_id} is done!");
    // }
    for _ in 1..=threads_num {
        let _ = rx.recv().unwrap();
    }

    let elapsed = start.elapsed().unwrap();
    println!("All processes completed, counter: {counter:?}, elapsed: {elapsed:?}");
}
