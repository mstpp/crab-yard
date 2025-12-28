use std::io::{BufRead, BufReader};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7575")?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut reader = BufReader::new(&mut stream);

        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            println!("Received msg: {}", line.trim());
            line.clear();
        }
    }
    Ok(())
}
