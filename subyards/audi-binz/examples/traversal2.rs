use std::collections::VecDeque;
use std::io;
use std::path::{Path, PathBuf};

fn files_dirs(
    path: &Path,
    files: &mut Vec<PathBuf>,
    dirs: &mut VecDeque<PathBuf>,
) -> io::Result<()> {
    for dir_entry in path.read_dir()? {
        let path_entry = dir_entry?.path();
        if path_entry.is_dir() {
            dirs.push_back(path_entry);
        } else if path_entry.is_file() {
            files.push(path_entry);
        } else {
            println!("not file nor dir: {:?}", path_entry);
        }
    }
    Ok(())
}

fn traverse<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    // BFS (FIFO)
    let mut queue: VecDeque<PathBuf> = VecDeque::from([path.as_ref().to_path_buf()]);
    while let Some(dir_path) = queue.pop_front() {
        files_dirs(&dir_path, &mut files, &mut queue)?;
    }
    Ok(files)
}

fn main() -> io::Result<()> {
    let files = traverse("dir-tree-example")?;
    for f in files {
        println!("{}", f.to_string_lossy());
    }
    Ok(())
}
