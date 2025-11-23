#![allow(dead_code, unused_variables)]
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
enum FsNode {
    File {
        name: String,
        size: u64,
    },
    Directory {
        name: String,
        children: RefCell<Vec<Rc<FsNode>>>,
    },
}

fn create_file(name: &str, size: u64) -> Rc<FsNode> {
    Rc::new(FsNode::File {
        name: name.to_string(),
        size: size,
    })
}

fn create_directory(name: &str) -> Rc<FsNode> {
    Rc::new(FsNode::Directory {
        name: name.to_string(),
        children: RefCell::new(Vec::new()),
    })
}

// Add a child to a directory (supports adding the same file/dir multiple times)
fn add_child(parent: &Rc<FsNode>, child: Rc<FsNode>) {
    match parent.as_ref() {
        FsNode::Directory { name, children } => children.borrow_mut().push(child),
        FsNode::File { name, size } => println!("can't add child to a file"),
    }
}

fn calculate_size(node: &Rc<FsNode>) -> u64 {
    let mut total = 0;
    let mut seen: HashSet<usize> = HashSet::new(); // Store pointer addresses
    let mut stack = vec![Rc::clone(node)];

    while let Some(current) = stack.pop() {
        match current.as_ref() {
            FsNode::File { name, size } => {
                let addr = Rc::as_ptr(&current) as usize;
                if seen.insert(addr) {
                    // Returns true if newly inserted
                    total += size;
                }
            }
            FsNode::Directory { children, .. } => {
                stack.extend(children.borrow().iter().map(Rc::clone));
            }
        }
    }
    total
}

fn find_all_paths(root: &Rc<FsNode>, target_name: &str) -> Vec<String> {
    let mut paths = Vec::new();

    let FsNode::Directory {
        name: root_name,
        children,
    } = root.as_ref()
    else {
        return paths;
    };

    let mut stack: Vec<(Rc<FsNode>, String)> = children
        .borrow()
        .iter()
        .map(|n| (Rc::clone(n), root_name.clone()))
        .collect();

    while let Some((current, path)) = stack.pop() {
        match current.as_ref() {
            FsNode::File { name, .. } => {
                if name == target_name {
                    paths.push(format!("{path}/{name}"));
                }
            }
            FsNode::Directory { name, children } => {
                let new_path = format!("{path}/{name}");
                stack.extend(
                    children
                        .borrow()
                        .iter()
                        .map(|n| (Rc::clone(n), new_path.clone())),
                );
            }
        }
    }
    paths
}

// fn find_all_paths(root: &Rc<FsNode>, target_name: &str) -> Vec<String> {
//     let mut paths: Vec<String> = vec![];
//     let mut lifo: Vec<(Rc<FsNode>, String)> = Vec::new();

//     match root.as_ref() {
//         FsNode::File { name, size } => return paths,
//         FsNode::Directory { name, children } => {
//             lifo.extend(
//                 children
//                     .borrow()
//                     .clone()
//                     .into_iter()
//                     .map(|n| (n, name.to_string())),
//             );
//         }
//     }

//     // LIFO traversing - DFS
//     while !lifo.is_empty() {
//         let (current, cwd) = lifo.pop().unwrap();
//         match current.as_ref() {
//             FsNode::File { name, size } => {
//                 if name == target_name {
//                     paths.push(format!("{cwd}/{name}"));
//                 }
//             }
//             FsNode::Directory { name, children } => {
//                 lifo.extend(
//                     children
//                         .borrow()
//                         .clone()
//                         .into_iter()
//                         .map(|n| (n, format!("{}/{}", &cwd, &name))),
//                 );
//             }
//         }
//     }
//     paths
// }

// Count how many references exist to a node
fn reference_count(node: &Rc<FsNode>) -> usize {
    Rc::strong_count(node)
}

// 3. Test with this scenario:
fn main() {
    // root/
    // |---/docs/report.pdf
    // |---/docs/mydocs/report.pdf
    // |---/projects/report.pdf
    //
    //
    let root = create_directory("root");
    let docs = create_directory("documents");
    let mydocs = create_directory("mydocs");
    let projects = create_directory("projects");

    // Create a large file
    let important_file = create_file("report.pdf", 1024);

    // Add the same file to multiple locations (like symbolic links)
    add_child(&docs, Rc::clone(&important_file));
    add_child(&docs, Rc::clone(&mydocs));
    add_child(&mydocs, Rc::clone(&important_file));
    add_child(&projects, Rc::clone(&important_file));

    add_child(&root, docs);
    add_child(&root, projects);

    // Should count report.pdf only once, not twice!
    println!("Total size: {}", calculate_size(&root));

    // Should find 2 paths to report.pdf
    println!("Paths: {:?}", find_all_paths(&root, "report.pdf"));

    // Should show 3 references (original + 2 locations)
    println!("References: {}", reference_count(&important_file));
}

// Result:
//
// Total size: 1024
// Paths: ["root/projects/report.pdf", "root/documents/mydocs/report.pdf", "root/documents/report.pdf"]
// References: 4
