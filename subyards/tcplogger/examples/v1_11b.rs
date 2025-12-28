// Bad example: buffer only process 10 bytes,
// so any larger message gets lost

use std::io::Read;
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7575")?;

    // small 10 bytes buffer
    let mut buf: [u8; 10] = [0; 10]; // 10 bytes + last new line 

    for stream in listener.incoming() {
        let mut data = stream?; // bad: stop server on single connection error
        let bytes_read = data.read(&mut buf)?; // ditto
        println!("Read from stream: {bytes_read} bytes");
        // BAD: if print whole butter it would leave the previous buf data
        // let utf8_encoded = String::from_utf8_lossy(&buf);
        let utf8_encoded = String::from_utf8_lossy(&buf[..bytes_read]);
        println!("Received msg: {}", utf8_encoded.trim());
    }

    Ok(())
}
