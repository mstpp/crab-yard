use clap::Parser;
use std::io::BufRead;
use std::io::Write;

#[derive(Debug, Parser)]
struct Cli {
    name: String,
    #[arg(short, long)]
    number: bool,
    #[arg(short, long)]
    utf8: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("{:?}", &cli);
    let f = std::fs::File::open(cli.name)?;
    let r = std::io::BufReader::new(f);
    let mut w = std::io::BufWriter::new(std::io::stdout());

    if cli.utf8 {
        for (n, line) in r.split(b'\n').enumerate() {
            let l = line?;
            if cli.number {
                write!(w, "{} ", n + 1)?;
            }
            w.write_all(&l)?;
            writeln!(w)?;
        }
    } else {
        for (n, line) in r.lines().enumerate() {
            let l = line?;
            if cli.number {
                write!(w, "{}: ", n + 1)?;
            }
            writeln!(w, "{}", l)?;
        }
    }

    Ok(())
}
