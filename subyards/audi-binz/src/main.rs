// 1. traverse from input path to all subdirs
// 2. check file type and matadata,
// 3. aggragate files per extension, collect size, lines count and file count per extension
// 4. display in nice table view
//

mod ext;
use crate::ext::ExtAgg; // File Extension Type
use content_inspector::inspect;
use std::env;
use std::error::Error;
use std::fmt;
use std::path::Path;

#[derive(Debug)]
enum CliError {
    MissingArgument,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::MissingArgument => {
                write!(f, "required exactly 1 argument: <path>")
            }
        }
    }
}

impl Error for CliError {}

fn process_files<P: AsRef<Path>>(path: P, stats: &mut ExtAgg) {
    let mut stack = vec![path.as_ref().to_path_buf()];
    while let Some(path) = stack.pop() {
        for item in path.read_dir().unwrap() {
            let dir = item.unwrap();
            let path = dir.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.is_file() {
                // get size and ext and push to stats
                let metadata = path.metadata().unwrap();
                let size = metadata.len();
                // bin vs txt - expensive, reading a file
                let content = std::fs::read(&path).unwrap();
                let bin_file = inspect(&content).is_binary();

                if let Some(ext) = path.extension() {
                    let extension = ext.to_string_lossy().to_string();
                    stats.add_file(extension, size, bin_file);
                } else {
                    if bin_file {
                        stats.add_file("no-ext-bin".to_string(), size, bin_file);
                    } else {
                        stats.add_file("no-ext-txt".to_string(), size, bin_file);
                    }
                }
            } else {
                println!("Special file, not processed: {}", path.display());
            }
        }
    }
}

fn run() -> Result<(), CliError> {
    let mut a = env::args();
    let mut stats = ExtAgg::new();

    if let Some(p) = a.nth(1) {
        process_files(p, &mut stats);
    } else {
        return Err(CliError::MissingArgument);
    }

    stats.display();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {e}");
        std::process::exit(1);
    }
}
