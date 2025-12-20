// 1. traverse from input path to all subdirs
// 2. check file type and matadata,
// 3. aggragate files per extension, collect size, lines count and file count per extension
// 4. display in nice table view
//

mod ext;
use crate::ext::ExtType;
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

fn run() -> Result<(), CliError> {
    let mut a = env::args();

    if let Some(p) = a.nth(1) {
        let mut agr: Vec<ExtType> = vec![];
        let start_path = Path::new(&p);
    } else {
        return Err(CliError::MissingArgument);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {e}");
        std::process::exit(1);
    }
}
