use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

fn main() {
    let path = "dir-tree-example";
    let p = Path::new(path);

    // DFS with stack/vec
    let mut dir_stack: Vec<PathBuf> = vec![];
    let mut fil_stack: Vec<PathBuf> = vec![];
    // initiall dir stack filling
    for item in p.read_dir().expect("read dir failed") {
        let i: DirEntry = item.expect("bad dir entry");
        if i.path().is_dir() {
            dir_stack.push(i.path());
        } else {
            fil_stack.push(i.path());
        }
        println!("Found item: {i:?}");
    }
    println!("Initial dir stack: {dir_stack:?}");

    // use idiomati while let Some()
    while !dir_stack.is_empty() {
        let current = dir_stack.pop().unwrap();
        for item in current.read_dir().expect("read dir failed") {
            let i: DirEntry = item.expect("bad dir entry");
            if i.path().is_dir() {
                dir_stack.push(i.path());
                println!("adding to dir stack: {i:?}");
            } else {
                fil_stack.push(i.path());
            }
            println!("Found item: {i:?}");
        }
    }

    println!("Files found {:?}", fil_stack);
}
