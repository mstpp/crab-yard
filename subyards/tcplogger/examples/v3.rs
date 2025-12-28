use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7575")?;
    println!("Server listening on 127.0.0.1:7575");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    eprintln!("Error handling connection: {e}");
                }
            }
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("New connection from {peer_addr}");

    let mut line = String::new();

    loop {
        let bytes = {
            let mut reader = BufReader::new(&stream);
            reader.read_line(&mut line)?
        };
        print!("Received: {}", line);

        if bytes == 0 {
            break;
        }

        // Echo back to client
        stream.write_all(b"Message received\n")?;
        line.clear();
    }

    println!("Connection from {peer_addr} closed");

    Ok(())
}
