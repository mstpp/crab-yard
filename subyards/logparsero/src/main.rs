use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader, BufWriter};

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("usage: log_peeker <path/to/logfile>");

    let file = File::open(&path).unwrap();
    let wfile = File::create("parsed.log").unwrap();
    let mut lines_count = 0;

    let mut reader = BufReader::with_capacity(1024 * 1024, file); // 1MiB
    let mut writer = BufWriter::new(wfile);

    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).unwrap();
        if bytes_read == 0 {
            break;
        } //EOF

        let mut line_split = line.splitn(4, " ");
        let ldate = line_split.next().unwrap_or("no-date");
        let ltime = line_split.next().unwrap_or("no-time");
        let ltype = line_split.next().unwrap_or("no-type");
        let lmsg = line_split.next().unwrap_or("no-msg");

        lines_count += 1;
        let _ = write!(writer, "{ldate}-{ltime} ::: {ltype} ::: {lmsg}");
    }
    let _ = writer.flush();

    println!("Lines counted in {path}: {lines_count}");
}
