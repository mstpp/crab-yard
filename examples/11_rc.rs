// When to use it?
// >> multiple owners - no single owner with longest lifetime
//
// Usage examples:
// - Tree Structures with Multiple Parents
// - Graph Data Structures
// - Shared Configuration Objects
// - Shared State
// - Caching/Memoization
// - Iterators and Closures Capturing the Same Environment

use std::rc::Rc;

#[derive(Debug)]
struct Node {
    msg: String,
    #[allow(dead_code)]
    parent: Option<Rc<Node>>,
}
fn main() -> anyhow::Result<()> {
    // since Rc is immutable, we can't modify parent
    // adding new nodes from oldest, to newest
    // 1 father > many children
    let adam = Rc::new(Node {
        msg: "I am born first, call me Adam".to_string(),
        parent: None,
    });
    let cain = Rc::new(Node {
        msg: "1st son of Adam".to_string(),
        parent: Some(Rc::clone(&adam)),
    });
    let abel = Rc::new(Node {
        msg: "Cain's little bro".to_string(),
        parent: Some(Rc::clone(&adam)),
    });
    let seth = Rc::new(Node {
        msg: "Third child".to_string(),
        parent: Some(Rc::clone(&adam)),
    });

    println!("Adam: {:?}, ref count: {}", &adam, Rc::strong_count(&adam));
    println!("Cain: {:?}, ref count: {}", &cain, Rc::strong_count(&cain));
    println!("Abel: {:?}, ref count: {}", &abel, Rc::strong_count(&abel));
    println!("Seth: {:?}, ref count: {}", &seth, Rc::strong_count(&seth));

    println!("Message from Adam: {:?}", adam.msg);

    Ok(())
}
