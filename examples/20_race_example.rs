use std::sync::{Arc, Mutex};
use std::thread;

fn no_race_example() {
    let c = Arc::new(Mutex::new(0_usize));
    let mut children = vec![];

    for i in 1..=3 {
        let counter = c.clone();
        children.push(thread::spawn(move || {
            println!("Spawned thread: {i}");
            let mut lock = counter.lock().unwrap();
            *lock += 1;
        }));
    }

    for child in children {
        let _ = child.join();
    }

    println!("Counter: {c:?}");
}

fn race_example() {
    let mut c: i64 = 0;
    let ptr = &mut c as *mut i64 as usize; // cast to usize - it's Send!

    thread::scope(|s| {
        for _ in 0..1000 {
            s.spawn(move || {
                for _ in 0..1000 {
                    unsafe {
                        let ptr = ptr as *mut i64;
                        *ptr = *ptr + 1;
                    }
                }
            });
        }
    });

    println!("Expected: {}", 1000 * 1000);
    println!("Actual:   {}", c);
}

/*
Expected: 1000000
Actual:   889093
*/

fn main() {
    no_race_example();
    race_example();
}
