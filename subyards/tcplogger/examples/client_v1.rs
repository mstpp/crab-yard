use std::io::{BufRead, BufReader, BufWriter};
use std::thread::{self, JoinHandle, sleep};
use std::time::Duration;
use std::{io::Write, net::TcpStream};

type TcpMsgResult = std::io::Result<(usize, String)>;

fn send_tcp_msg(msg: &str) -> TcpMsgResult {
    let stream = TcpStream::connect("127.0.0.1:7575")?;

    stream.set_write_timeout(Some(Duration::from_secs(2)))?;

    if let Some(timeout) = stream.write_timeout()? {
        println!("Timeout: {:?}", timeout);
    } else {
        println!("Couldn't get timeout value");
    };

    let mut writer = BufWriter::new(stream.try_clone()?);
    write!(&mut writer, "{}", msg)?;
    // random wait 0-10s
    let sleep_time = rand::random_range(0..=100) as f64 * 0.1;
    sleep(Duration::from_secs_f64(sleep_time));
    writeln!(&mut writer, "After sleeping {} seconds", sleep_time)?;
    writer.flush()?;

    // response
    let mut reader = BufReader::new(stream);
    let mut buf = String::new();
    let got = reader.read_line(&mut buf)?;
    Ok((got, buf.trim().to_string()))
}

fn main() -> std::io::Result<()> {
    let mut threads: Vec<JoinHandle<TcpMsgResult>> = Vec::new();
    for i in 1..=10 {
        let handle = thread::spawn(move || {
            let msg = format!("[{}] Hey Joe!", i);
            send_tcp_msg(&msg)
        });
        threads.push(handle);
    }

    // Wait for all threads and collect results
    for (i, handle) in threads.into_iter().enumerate() {
        match handle.join() {
            Ok(result) => match result {
                Ok((bytes, response)) => {
                    println!("Thread {} got {} bytes: {}", i, bytes, response);
                }
                Err(e) => eprintln!("Thread {} I/O error: {}", i, e),
            },
            Err(e) => eprintln!("Thread {} panicked: {:?}", i, e),
        }
    }

    Ok(())
}
