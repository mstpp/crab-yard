use rand::RngExt;
use std::sync::mpsc;

fn process(id: usize, tx: mpsc::Sender<usize>) {
    println!("[{id}] Process start");
    let mut rng = rand::rng();
    let millis = rng.random_range(1..20) * 100;
    println!("[{id}] Processing for {millis} ms");
    std::thread::sleep(std::time::Duration::from_millis(millis));
    println!("[{id}] Process finished");
    // Send the ID back to the main thread once finished
    tx.send(id).unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 1..=5 {
        let tx_clone = tx.clone(); // Each thread needs its own "sender"
        std::thread::spawn(move || process(i, tx_clone));
    }

    // Drop the original sender so the receiver knows no more messages are coming
    drop(tx);

    // This loop executes as soon as ANY thread sends a message
    for finished_id in rx {
        println!("Received notification: Process {finished_id} is done!");
    }

    println!("All processes completed.");
}
