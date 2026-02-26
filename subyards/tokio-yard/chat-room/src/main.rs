use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() {
    println!("Opened TCP server connection at localhost:7575");
    let listener = tokio::net::TcpListener::bind("localhost:7575")
        .await
        .unwrap(); // TODO err handling

    // each connection is new tokio async task
    loop {
        let (tcp_stream, socket_addr) = listener.accept().await.unwrap(); // TODO error handling 
        tokio::spawn(async move {
            println!("Connected to {socket_addr:?}");

            let reader = BufReader::new(tcp_stream);
            let mut lines = reader.lines();

            loop {
                match lines.next_line().await {
                    Ok(Some(msg)) => println!("got line: {msg}"),
                    Ok(None) => {
                        println!("Conn lost {socket_addr}");
                        break;
                    }
                    Err(e) => {
                        println!("conn err {e:?}");
                        break;
                    }
                }
            }
        });
    }
}
