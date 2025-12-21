// Os { code: 24, kind: Uncategorized, message: "Too many open files" }
use std::fs::DirEntry;
use std::path::Path;

fn main() {
    let mut v: Vec<DirEntry> = Vec::new();
    let path = Path::new("/tmp");
    // already opened at least 3 file descriptors
    // for stdin, stdout, stderr
    for i in 0..2560 - 2 {
        println!("{i}");
        for item in path.read_dir().unwrap() {
            let d = item.unwrap();
            v.push(d);
        }
    }
}
