use chrono;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

static LOG_FILE: &str = "tcp_server.log";

#[derive(Debug)]
struct Logger<T, F>
where
    T: Write,
    F: Write,
{
    term: T, // term for terminal, or stdout
    file: F,
}

type DualLogger = Logger<BufWriter<std::io::Stdout>, BufWriter<std::fs::File>>;

impl<T, F> Logger<T, F>
where
    T: Write,
    F: Write,
{
    pub fn new() -> std::io::Result<DualLogger> {
        let term = BufWriter::new(std::io::stdout());
        let file = BufWriter::new(File::create(LOG_FILE)?);

        Ok(Logger { term, file })
    }

    fn log_msg(&mut self, msg: &str) -> std::io::Result<()> {
        writeln!(self.term, "{}", msg)?;
        self.term.flush()?;
        writeln!(self.file, "{}", msg)?;
        self.file.flush()?;
        Ok(())
    }

    pub fn info(&mut self, msg: &str) -> std::io::Result<()> {
        let now = chrono::Utc::now();
        let time_msg = format!("{} INFO {}", now.format("%Y-%m-%d %H:%M:%S%.3f"), msg);
        self.log_msg(&time_msg)?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut logger = DualLogger::new()?;
    let listener = TcpListener::bind("127.0.0.1:7575")?;
    logger.info("Server listening on 127.0.0.1:7575")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream, &mut logger) {
                    eprintln!("Error handling connection: {e}");
                }
            }
            Err(e) => eprintln!("Connection failed: {e}"),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, logger: &mut DualLogger) -> std::io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    logger.info(format!("New connection from {}", peer_addr).as_str())?;

    let mut reader = BufReader::new(&mut stream);
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        logger.info(format!("Received: {}", line.trim()).as_str())?;
        reader.get_mut().write_all(b"Message received\n")?;
        line.clear();
    }

    logger.info(format!("Connection from {} closed", peer_addr).as_str())?;

    Ok(())
}
