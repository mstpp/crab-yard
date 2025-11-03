use anyhow::Result;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

pub fn read_file_lines() -> Result<()> {
    // --- Read file line by line ---
    let file = std::fs::File::open("example.txt")?;
    let reader = BufReader::new(file);

    println!("File contents:");
    for line_result in reader.lines() {
        let line = line_result?;
        println!("{}", line);
    }

    Ok(())
}

pub fn append_line() -> Result<()> {
    let path = "example.txt";
    let mut file = OpenOptions::new().append(true).create(true).open(path)?;
    writeln!(file, "This is a new line appended at the end!")?;
    println!("\nNew line appended successfully.");
    Ok(())
}

fn main() {
    println!("=========================");
    let _ = read_file_lines();
    let _ = append_line();
}
