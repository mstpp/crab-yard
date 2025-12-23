use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let path = args.next().unwrap();
    let file = File::open(path).unwrap();
    let mut reader = BufReader::with_capacity(128 * 1024, file);
    let mut line = String::new();

    // stats
    let mut lines = 0;
    let mut chars = 0;
    let mut bytes = 0;
    let mut words = 0;

    loop {
        let read = reader.read_line(&mut line).unwrap();
        if read == 0 {
            break;
        } //EOF

        lines += 1;
        bytes += line.len();
        chars += line.chars().count();
        words += line.split_whitespace().count();

        // println!("{lines}: {line}");
        line.clear();
    }

    println!("Lines count: {lines}");
    println!("Chars count: {chars}");
    println!("Bytes count: {bytes}");
    println!("Words count: {words}");
}
